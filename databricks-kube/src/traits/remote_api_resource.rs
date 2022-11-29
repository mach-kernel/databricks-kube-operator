use std::{fmt::Debug, hash::Hash, pin::Pin, sync::Arc, time::Duration};

use crate::{
    context::Context, error::DatabricksKubeError,
    util::default_error_policy,
};

use assert_json_diff::assert_json_matches_no_panic;
use futures::{Future, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt};

use k8s_openapi::{DeepMerge, NamespaceResourceScope};

use kube::{
    api::ListParams,
    api::PostParams,
    runtime::{controller::Action, reflector::ObjectRef, watcher, watcher::Event, Controller},
    Api, CustomResourceExt, Resource, ResourceExt,
};

use serde::{de::DeserializeOwned, Serialize};
use tokio::{task::JoinHandle, time::interval};

/// Generic sync task for pushing remote API resources into K8S
/// TAPIType is OpenAPI generated
/// TCRDType is the operator's wrapper
#[allow(dead_code)]
pub async fn ingest_task<TAPIType, TCRDType>(
    interval_period: Duration,
    context: Arc<Context>,
) -> Result<(), DatabricksKubeError>
where
    TCRDType: From<TAPIType>,
    TCRDType: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
    TCRDType::DynamicType: Default + Eq + Hash,
    TCRDType: RemoteAPIResource<TAPIType>,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: RemoteAPIResource<TAPIType>,
    TCRDType: 'static,
    TAPIType: Send,
    TAPIType: 'static,
{
    let mut duration = interval(interval_period);
    let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());

    loop {
        duration.tick().await;

        log::info!(
            "Looking for uningested {}(s)",
            TCRDType::api_resource().kind
        );

        let mut resource_stream = TCRDType::remote_list_all(context.clone());

        while let Ok(Some(api_resource)) = resource_stream.try_next().await {
            let mut resource_as_kube: TCRDType = api_resource.into();
            let name = resource_as_kube.name_unchecked();
            let kube_resource = kube_api.get(&name).await;

            if kube_resource.is_err() {
                log::info!(
                    "{} missing, creating {}",
                    TCRDType::api_resource().kind,
                    name
                );
            }

            resource_as_kube
                .annotations_mut()
                .insert("databricks-operator/owner".to_string(), "api".to_string());

            if let Ok(ref new_kube_resource) = kube_api
                .create(&PostParams::default(), &resource_as_kube)
                .await
            {
                log::info!(
                    "Created {} {}",
                    TCRDType::api_resource().kind,
                    new_kube_resource.name_unchecked(),
                );
            }
        }
        log::info!("{} ingest complete", TCRDType::api_resource().kind);
    }
}

#[allow(dead_code)]
pub async fn spawn_delete_watcher<TAPIType, TCRDType>(
    resource: Arc<TCRDType>,
    context: Arc<Context>,
) -> JoinHandle<Result<(), DatabricksKubeError>>
where
    TCRDType: From<TAPIType>,
    TCRDType: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
    TCRDType::DynamicType: Default + Eq + Hash,
    TCRDType: RemoteAPIResource<TAPIType>,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: 'static,
    TAPIType: Send,
    TAPIType: 'static,
{
    let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());

    let params = ListParams {
        field_selector: Some(format!("metadata.name={}", resource.name_unchecked())),
        ..ListParams::default()
    };

    tokio::spawn(async move {
        let mut watcher = watcher::watcher(kube_api, params).boxed();

        while let Some(event) = watcher
            .try_next()
            .map_err(|_e| DatabricksKubeError::ConfigMapMissingError)
            .await?
        {
            if let Event::Deleted(r) = event {
                let owner = r
                    .annotations()
                    .get("databricks-operator/owner")
                    .map(Clone::clone)
                    .unwrap_or("operator".to_string());

                if owner == "operator" {
                    log::info!(
                        "Removing {} {} from Databricks",
                        TCRDType::api_resource().kind,
                        resource.name_unchecked()
                    );

                    resource.remote_delete(context.clone()).next().await;

                    log::info!(
                        "Removed {} {} from Databricks",
                        TCRDType::api_resource().kind,
                        resource.name_unchecked()
                    );
                }

                let watchers = context.delete_watchers.pin();
                let handle = &**watchers.remove(&resource.self_url_unchecked()).unwrap();
                handle.abort();

                return Ok(());
            }
        }

        Ok(())
    })
}

#[allow(dead_code)]
async fn reconcile<TAPIType, TCRDType>(
    resource: Arc<TCRDType>,
    context: Arc<Context>,
) -> Result<Action, DatabricksKubeError>
where
    TCRDType: From<TAPIType>,
    TCRDType: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
    TCRDType::DynamicType: Default + Eq + Hash,
    TCRDType: RemoteAPIResource<TAPIType>,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: 'static,
    TAPIType: From<TCRDType>,
    TAPIType: PartialEq,
    TAPIType: Send,
    TAPIType: Serialize,
    TAPIType: 'static,
{
    let mut resource = resource;
    let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());
    let latest_remote = resource.remote_get(context.clone()).next().await.unwrap();

    // todo: enum
    let owner = resource
        .annotations()
        .get("databricks-operator/owner")
        .map(Clone::clone)
        .unwrap_or("operator".to_string());

    // Create if owned
    if (owner == "operator") && latest_remote.is_err() {
        log::info!(
            "Resource {} {} is missing in Databricks, creating",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        let created = resource
            .remote_create(context.clone())
            .next()
            .await
            .unwrap()?;

        log::info!(
            "Created {} {} in Databricks",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        kube_api
            .replace(&resource.name_unchecked(), &PostParams::default(), &created)
            .await
            .map_err(|e| DatabricksKubeError::ResourceUpdateError(e.to_string()))?;

        log::info!(
            "Updated {} {} in K8S",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        // Reconcile after create to bind things like the delete watcher
        return Ok(Action::requeue(Duration::from_secs(5)));
    }

    let latest_remote = latest_remote?;
    let kube_as_api: TAPIType = resource.as_ref().clone().into();

    // If resource in sync, spawn a task to watch for deletion events
    if (owner == "operator")
        && latest_remote == kube_as_api
        && !context
            .delete_watchers
            .pin()
            .contains_key(&resource.name_unchecked())
    {
        let handle = spawn_delete_watcher(resource.clone(), context.clone()).await;
        context
            .delete_watchers
            .pin()
            .insert(resource.self_url_unchecked(), Box::new(handle));
    }

    if latest_remote != kube_as_api {
        log::info!(
            "Resource {} {} drifted!\nDiff (remote, kube):\n{}",
            TCRDType::api_resource().kind,
            resource.name_unchecked(),
            assert_json_matches_no_panic(
                &latest_remote,
                &kube_as_api,
                assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict)
            )
            .unwrap_err()
        );
    }

    // Push to API if operator owned, or let user know
    if (latest_remote != kube_as_api) && (owner == "operator") {
        log::info!(
            "Resource {} {} is owned by databricks-kube-operator, reconciling drift...",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        let updated = resource
            .remote_update(context.clone())
            .next()
            .await
            .unwrap()?;

        let replaced = kube_api
            .replace(&resource.name_unchecked(), &PostParams::default(), &updated)
            .await
            .map_err(|e| DatabricksKubeError::ResourceUpdateError(e.to_string()))?;

        resource = replaced.into();

        log::info!(
            "Updated {} {} in K8S",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );
    } else if latest_remote != kube_as_api {
        log::info!(
            "Resource {} {} is not owned by databricks-kube-operator, updating Kubernetes object.\nIngested resources are databricks-operator/owner: api\nTo push updates to Databricks, ensure databricks-operator/owner: operator by creating your object in Kubernetes first.",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        let mut latest_as_kube: TCRDType = latest_remote.into();
        latest_as_kube
            .annotations_mut()
            .merge_from(resource.annotations().clone());
        latest_as_kube
            .labels_mut()
            .merge_from(resource.labels().clone());
        latest_as_kube
            .meta_mut()
            .merge_from(resource.meta().clone());

        let replaced = kube_api
            .replace(
                &resource.name_unchecked(),
                &PostParams::default(),
                &latest_as_kube,
            )
            .await
            .map_err(|e| DatabricksKubeError::ResourceUpdateError(e.to_string()))?;

        resource = replaced.into();
    }

    if owner == "operator" {
        resource.every_reconcile_owned(context.clone()).await?;
    }

    Ok(Action::requeue(Duration::from_secs(300)))
}

/// Implement this on the macroexpanded CRD type, against the SDK type
pub trait RemoteAPIResource<TAPIType: 'static> {
    fn controller(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(ObjectRef<Self>, Action), DatabricksKubeError>> + Send>>
    where
        Self: From<TAPIType>,
        Self: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
        Self::DynamicType: Clone + Debug + Default + Eq + Hash + Unpin,
        Self: RemoteAPIResource<TAPIType>,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: 'static,
        TAPIType: From<Self>,
        TAPIType: PartialEq,
        TAPIType: Send,
        TAPIType: Serialize,
        TAPIType: 'static,
    {
        let root_kind_api = Api::<Self>::default_namespaced(context.client.clone());

        Controller::new(root_kind_api.clone(), ListParams::default())
            .shutdown_on_signal()
            .run(reconcile, default_error_policy, context.clone())
            .map_err(|e| DatabricksKubeError::ControllerError(e.to_string()))
            .boxed()
    }

    fn ingest_task(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Result<(), DatabricksKubeError>> + Send + 'static>>
    where
        Self: From<TAPIType>,
        Self: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
        Self::DynamicType: Default + Eq + Hash,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: 'static,
        TAPIType: Send,
    {
        ingest_task::<TAPIType, Self>(Duration::from_secs(300), context).boxed()
    }

    fn self_url_unchecked(&self) -> String
    where
        Self: Resource + ResourceExt,
        Self::DynamicType: Default + Eq + Hash,
    {
        let ns = self.namespace().unwrap();
        format!(
            "{}/{}",
            Self::url_path(&Default::default(), Some(&ns)),
            self.name_unchecked()
        )
    }

    fn every_reconcile_owned(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DatabricksKubeError>> + Send>> {
        async { Ok(()) }.boxed()
    }

    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;

    fn remote_create(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized;

    fn remote_update(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized;

    fn remote_delete(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>>;
}
