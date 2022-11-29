use std::{fmt::Debug, hash::Hash, pin::Pin, sync::Arc, time::Duration};

use crate::{
    context::Context, error::DatabricksKubeError,
    traits::rest_config::RestConfig,
    util::default_error_policy,
};

use futures::{Future, Stream, StreamExt, TryStreamExt};

use k8s_openapi::{DeepMerge, NamespaceResourceScope};

use kube::{
    api::ListParams,
    api::PostParams,
    core::object::HasStatus,
    runtime::{controller::Action, reflector::ObjectRef, Controller},
    Api, CustomResourceExt, Resource, ResourceExt,
};

use serde::{de::DeserializeOwned, Serialize};


#[allow(dead_code)]
async fn reconcile<TStatusType, TCRDType>(
    resource: Arc<TCRDType>,
    context: Arc<Context>,
) -> Result<Action, DatabricksKubeError>
where
    TCRDType: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
    TCRDType: HasStatus<Status = TStatusType>,
    TCRDType: RemoteAPIStatus<TStatusType>,
    TCRDType::DynamicType: Default + Eq + Hash,
    TCRDType: Send,
    TCRDType: Serialize,
    TCRDType: Sync,
    TCRDType: Default,
    TCRDType: Clone,
    TCRDType: CustomResourceExt,
    TCRDType: Debug,
    TCRDType: DeserializeOwned,
    TCRDType: 'static,
    TStatusType: DeepMerge,
    TStatusType: PartialEq,
    TStatusType: Send,
    TStatusType: Serialize,
    TStatusType: 'static,
{
    let resource = resource;
    let kube_api = Api::<TCRDType>::default_namespaced(context.client.clone());

    log::info!(
        "Checking status for {} {}",
        TCRDType::api_resource().kind,
        resource.name_unchecked()
    );

    let latest_status = resource.remote_status(context.clone()).await?;

    log::info!(
        "Updating status for {} {}",
        TCRDType::api_resource().kind,
        resource.name_unchecked()
    );

    let mut updated_resource = resource.as_ref().clone();
    updated_resource
        .status_mut()
        .merge_from(latest_status);

    kube_api
        .replace(
            &resource.name_unchecked(),
            &PostParams::default(),
            &updated_resource,
        )
        .await
        .map_err(|e| DatabricksKubeError::ResourceUpdateError(e.to_string()))?;

    log::info!(
        "Updated status for {} {}",
        TCRDType::api_resource().kind,
        resource.name_unchecked()
    );

    Ok(Action::requeue(Duration::from_secs(300)))
}

pub trait RemoteAPIStatus<TStatusType: 'static> {
    fn status_controller(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(ObjectRef<Self>, Action), DatabricksKubeError>> + Send>>
    where
        Self: Resource<Scope = NamespaceResourceScope> + ResourceExt + CustomResourceExt,
        Self: HasStatus<Status = TStatusType>,
        Self::DynamicType: Clone + Debug + Default + Eq + Hash + Unpin,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        Self: Default,
        Self: Clone,
        Self: CustomResourceExt,
        Self: Debug,
        Self: DeserializeOwned,
        Self: 'static,
        TStatusType: DeepMerge,
        TStatusType: PartialEq,
        TStatusType: Send,
        TStatusType: Serialize,
    {
        let root_kind_api = Api::<Self>::default_namespaced(context.client.clone());
        Controller::new(root_kind_api.clone(), ListParams::default())
            .shutdown_on_signal()
            .run(reconcile, default_error_policy, context.clone())
            .map_err(|e| DatabricksKubeError::ControllerError(e.to_string()))
            .boxed()
    }

    fn remote_status(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Future<Output = Result<Option<TStatusType>, DatabricksKubeError>> + Send>>;
}
