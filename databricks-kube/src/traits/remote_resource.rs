use crate::{config::Config, crds::databricks_job::DatabricksJobSpec, error::DatabricksKubeError};
use async_stream::try_stream;
use databricks_rust_jobs::{
    apis::{configuration::Configuration, default_api},
    models::{Job, JobsList200Response},
};
use futures::{FutureExt, Stream};
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::{Metadata, NamespaceResourceScope};
use kube::{
    api::PostParams,
    core::{DynamicObject, DynamicResourceScope},
    Api, Resource,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, pin::Pin, time::Duration};
use tokio::{task::JoinHandle, time::interval};

/// Implement this on the macroexpanded CRD type, against the SDK type
pub trait RemoteResource<TAPIType> {
    fn task_sync_remote_to_kube(config: Config) -> JoinHandle<()>
    where
        Self: From<TAPIType>,
        Self: Resource<DynamicType = Self, Scope = NamespaceResourceScope>,
        Self: Default,
        Self: Clone,
        Self: Debug,
        Self: DeserializeOwned,
        Self: Send,
        Self: Serialize,
        Self: Sync,
        TAPIType: Send,
    {
        tokio::spawn(async move {
            let mut duration = interval(Duration::from_secs(10));
            let kube_api = Api::<Self>::default_namespaced(config.client.clone());

            loop {
                duration.tick().await;

                let (url, token) = config.get_databricks_url_token().await.unwrap();
                let databricks_config = Configuration {
                    base_path: url,
                    bearer_access_token: Some(token),
                    ..Configuration::default()
                };

                let mut resource_stream = Self::remote_list_all(databricks_config);

                while let Ok(Some(api_resource)) = resource_stream.try_next().await {
                    let resource_as_kube: Self = api_resource.into();
                    let name = resource_as_kube.meta().name.clone().unwrap();

                    let kube_resource = kube_api.get(&name).await;
                    if kube_resource.is_err() {
                        log::info!(
                            "{} missing, creating {}",
                            Self::kind(&resource_as_kube),
                            name
                        );
                    }

                    if let Ok(new_kube_resource) = kube_api
                        .create(&PostParams::default(), &resource_as_kube)
                        .await
                    {
                        log::info!(
                            "Created {} {}",
                            Self::kind(&new_kube_resource),
                            new_kube_resource.meta().name.clone().unwrap()
                        );
                    }
                }
            }
        })
    }

    fn remote_list_all(
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;
}
