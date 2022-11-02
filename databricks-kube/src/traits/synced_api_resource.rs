use crate::{context::Context, error::DatabricksKubeError};
use crate::traits::rest_config::RestConfig;


use futures::Stream;
use futures::TryStreamExt;
use k8s_openapi::NamespaceResourceScope;

use kube::api::ListParams;
use kube::runtime::controller::Action;
use kube::runtime::Controller;


use kube::{api::PostParams, Api, CustomResourceExt, Resource};
use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use std::{fmt::Debug, pin::Pin, time::Duration};
use tokio::time::interval;

use futures::FutureExt;
use futures::StreamExt;
use kube::ResourceExt;
use std::sync::Arc;

/// Generic sync task for pushing remote API resources into K8S
/// TAPIType is OpenAPI generated
/// TCRDType is the operator's wrapper
/// TDynamic is variable CRD metadata type for kube::Resource (varies)
async fn ingest_task<TAPIType, TCRDType, TDynamic, TRestConfig>(
    interval_period: Duration,
    context: Context,
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
            let resource_as_kube: TCRDType = api_resource.into();
            let name = resource_as_kube.name_unchecked();
            let kube_resource = kube_api.get(&name).await;

            if kube_resource.is_err() {
                log::info!(
                    "{} missing, creating {}",
                    TCRDType::api_resource().kind,
                    name
                );
            }

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
    TRestConfig: Clone,
    TRestConfig: Send,
    TRestConfig: Sync
{
    log::info!(
        "Reconciling {} {}",
        TCRDType::api_resource().kind,
        resource.name_unchecked()
    );
    // let rest_config = TAPIType::get_rest_config(context.as_ref().clone()).await;

    // if rest_config.is_none() {
    //     return Ok(Action::requeue(Duration::from_secs(15)));
    // }

    // let rest_config = rest_config.unwrap();
    let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());
    let latest_remote = resource
        .remote_get(context.as_ref().clone())
        .next()
        .await
        .unwrap();

    if latest_remote.is_err() {
        log::info!(
            "Resource {} {} exists in K8S but not API",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );
        log::info!(
            "Creating {} {} in Databricks",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        let created = resource.remote_create(context.as_ref().clone()).next().await.unwrap()?;

        log::info!(
            "Created {} {} in Databricks",
            TCRDType::api_resource().kind,
            resource.name_unchecked()
        );

        if let Ok(_r) = kube_api
            .replace(&resource.name_unchecked(), &PostParams::default(), &created)
            .await
        {
            log::info!(
                "Updated {} {} in K8S",
                TCRDType::api_resource().kind,
                resource.name_unchecked()
            );
        }
    }

    Ok(Action::await_change())
}

/// Implement this on the macroexpanded CRD type, against the SDK type
pub trait SyncedAPIResource<TAPIType: 'static, TRestConfig: Sync + Send + Clone> {
    fn spawn_controller<TDynamic>(
        context: Context,
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
        TRestConfig: Clone,
        TRestConfig: 'static,
    {
        let root_kind_api = Api::<Self>::default_namespaced(context.client.clone());

        Controller::new(root_kind_api, ListParams::default())
            .shutdown_on_signal()
            .run(reconcile, Self::default_error_policy, context.into())
            .for_each(|r| async move {
                match r {
                    Ok(_) => log::info!("Reconcile success!"),
                    Err(_) => log::info!("Reconcile fail!"),
                }
            })
            .map(|()| Ok(()))
            .boxed()
    }

    fn spawn_remote_ingest_task<TDynamic>(
        context: Context,
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
        TRestConfig: 'static
    {
        ingest_task::<TAPIType, Self, TDynamic, TRestConfig>(Duration::from_secs(60), context).boxed()
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
            "Reconciliation failed for {} {} -- with error {} -- retrying in 30s",
            Self::api_resource().kind,
            err,
            obj.name_unchecked()
        );
        Action::requeue(Duration::from_secs(30))
    }

    fn remote_list_all(
        context: Context,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;

    fn remote_get(
        &self,
        context: Context,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;

    fn remote_create(
        &self,
        context: Context,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized;
}
