use crate::traits::rest_config::RestConfig;
use crate::{context::Context, error::DatabricksKubeError};

use assert_json_diff::assert_json_matches_no_panic;
use futures::Stream;
use futures::TryStreamExt;
use k8s_openapi::NamespaceResourceScope;

use kube::api::{ListParams};
use kube::runtime::controller::Action;
use kube::runtime::watcher::Event;
use kube::runtime::{Controller, watcher, finalizer};

use kube::{api::PostParams, Api, CustomResourceExt, Resource};
use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use std::{fmt::Debug, pin::Pin, time::Duration};
use tokio::time::interval;

use futures::FutureExt;
use futures::StreamExt;
use kube::ResourceExt;
use std::sync::Arc;
use tokio::task::JoinHandle;
use futures::TryFutureExt;

/// Generic sync task for pushing remote API resources into K8S
/// TAPIType is OpenAPI generated
/// TCRDType is the operator's wrapper
/// TDynamic is variable CRD metadata type for kube::Resource (varies)
async fn ingest_task<TAPIType, TCRDType, TDynamic, TRestConfig>(
    interval_period: Duration,
    context: Arc<Context>,
) -> Result<(), DatabricksKubeError>
where
    TCRDType: From<TAPIType>,
    TCRDType: SyncedAPIResource<TAPIType, TRestConfig>,
    TCRDType: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: ResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TDynamic: Default,
    TDynamic: 'static,
    TAPIType: 'static,
    TAPIType: RestConfig<TRestConfig>,
    TRestConfig: Clone,
    TRestConfig: Send,
    TRestConfig: Sync,
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

async fn spawn_delete_watcher<TAPIType, TCRDType, TDynamic, TRestConfig>(
    resource: Arc<TCRDType>,
    context: Arc<Context>,
) -> JoinHandle<Result<(), DatabricksKubeError>>
where
    TCRDType: From<TAPIType>,
    TCRDType: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TCRDType: SyncedAPIResource<TAPIType, TRestConfig>,
    TCRDType: 'static,
    TDynamic: Default,
    TDynamic: Debug,
    TDynamic: Clone,
    TDynamic: Unpin,
    TDynamic: Eq,
    TDynamic: Hash,
    TDynamic: 'static,
    TAPIType: Send,
    TAPIType: 'static,
    TAPIType: RestConfig<TRestConfig>,
    TAPIType: From<TCRDType>,
    TAPIType: PartialEq,
    TAPIType: Serialize,
    TRestConfig: Clone,
    TRestConfig: Send,
    TRestConfig: Sync, {
        let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());

        let params = ListParams {
            field_selector: Some(format!("metadata.name={}", resource.name_unchecked())),
            ..ListParams::default()
        };

        tokio::spawn(async move {
            let mut watcher = watcher::watcher(
                kube_api,
                params
            ).boxed();

            while let Some(event) = watcher.try_next().map_err(|_e| DatabricksKubeError::ConfigMapMissingError).await? {
                if let Event::Deleted(r) = event {
                    let owner = r
                        .annotations()
                        .get("databricks-operator/owner")
                        .map(Clone::clone)
                        .unwrap_or("operator".to_string());

                    if owner != "operator" {
                        break;
                    }

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

                    context.delete_watchers.pin().remove(&resource.name_unchecked());
                    return Ok(());
                }
            }

            Ok(())
        })
    }

async fn reconcile<TAPIType, TCRDType, TDynamic, TRestConfig>(
    resource: Arc<TCRDType>,
    context: Arc<Context>,
) -> Result<Action, DatabricksKubeError>
where
    TCRDType: From<TAPIType>,
    TCRDType: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TCRDType: SyncedAPIResource<TAPIType, TRestConfig>,
    TCRDType: 'static,
    TDynamic: Default,
    TDynamic: Debug,
    TDynamic: Clone,
    TDynamic: Unpin,
    TDynamic: Eq,
    TDynamic: Hash,
    TDynamic: 'static,
    TAPIType: Send,
    TAPIType: 'static,
    TAPIType: RestConfig<TRestConfig>,
    TAPIType: From<TCRDType>,
    TAPIType: PartialEq,
    TAPIType: Serialize,
    TRestConfig: Clone,
    TRestConfig: Send,
    TRestConfig: Sync,
{
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

        return Ok(Action::await_change());
    }

    let latest_remote = latest_remote.unwrap();
    let kube_as_api: TAPIType = resource.as_ref().clone().into();

    // If resource in sync, spawn a task to watch for deletion events
    if (owner == "operator") 
        && latest_remote == kube_as_api
        && !context.delete_watchers.pin().contains_key(&resource.name_unchecked()) {   
        spawn_delete_watcher(resource.clone(), context.clone()).await;
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

        let updated = resource.remote_update(context.clone()).next().await.unwrap()?;

        kube_api
            .replace(&resource.name_unchecked(), &PostParams::default(), &updated)
            .await
            .map_err(|e| DatabricksKubeError::ResourceUpdateError(e.to_string()))?;

        log::info!(
            "Updated {} {} in K8S",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );
    } else if latest_remote != kube_as_api {
        log::info!(
            "Resource {} {} is not owned by databricks-kube-operator!\nIngested resources are databricks-operator/owner: api\nCreate your resource with databricks-kube-operator to reconcile.",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );
    }

    Ok(Action::await_change())
}

/// Implement this on the macroexpanded CRD type, against the SDK type
pub trait SyncedAPIResource<TAPIType: 'static, TRestConfig: Sync + Send + Clone> {
    fn controller<TDynamic>(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Result<(), DatabricksKubeError>> + Send>>
    where
        Self: From<TAPIType>,
        Self: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: 'static,
        TDynamic: Default,
        TDynamic: Debug,
        TDynamic: Clone,
        TDynamic: Unpin,
        TDynamic: Eq,
        TDynamic: Hash,
        TDynamic: Send,
        TDynamic: Sync,
        TDynamic: 'static,
        TAPIType: Send,
        TAPIType: RestConfig<TRestConfig>,
        TAPIType: From<Self>,
        TAPIType: PartialEq,
        TAPIType: Serialize,
        TRestConfig: Clone,
        TRestConfig: 'static,
    {
        let root_kind_api = Api::<Self>::default_namespaced(context.client.clone());

        Controller::new(root_kind_api, ListParams::default())
            .shutdown_on_signal()
            .run(reconcile, Self::default_error_policy, context.into())
            .for_each(|r| async move {
                match r {
                    Ok((object, _)) => log::info!("{} reconciled", object.name),
                    Err(e) => log::info!("{}", e),
                }
            })
            .map(|()| Ok(()))
            .boxed()
    }

    fn ingest_task<TDynamic>(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Result<(), DatabricksKubeError>> + Send + 'static>>
    where
        Self: From<TAPIType>,
        Self: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: 'static,
        TDynamic: Default,
        TDynamic: 'static,
        TAPIType: Send,
        TAPIType: RestConfig<TRestConfig>,
        TRestConfig: 'static,
    {
        ingest_task::<TAPIType, Self, TDynamic, TRestConfig>(Duration::from_secs(60), context)
            .boxed()
    }

    fn default_error_policy<TDynamic>(
        obj: Arc<Self>,
        err: &DatabricksKubeError,
        _ctx: Arc<Context>,
    ) -> Action
    where
        Self: From<TAPIType>,
        Self: Resource<DynamicType = TDynamic, Scope = NamespaceResourceScope>,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: ResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: 'static,
        TDynamic: Default,
        TDynamic: Eq,
        TDynamic: Hash,
        TDynamic: 'static,
        TAPIType: Send,
        TAPIType: RestConfig<TRestConfig>,
    {
        log::error!(
            "Reconciliation failed for {} {} (retrying in 30s):\n{}",
            Self::api_resource().kind,
            obj.name_unchecked(),
            err,
        );
        Action::requeue(Duration::from_secs(30))
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
