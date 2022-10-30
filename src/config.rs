use k8s_openapi::{
    api::core::v1::ConfigMap,
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    runtime::wait::{await_condition, conditions},
    Api, Client,
};
use lazy_static::lazy_static;
use std::{env, time::Duration};
use tokio::time::timeout;

use crate::error::DatabricksKubeError;

lazy_static! {
    pub static ref CONFIGMAP_NAME: String =
        env::var("DATABRICKS_KUBE_CONFIGMAP").unwrap_or("databricks-kube-operator".to_owned());
}

pub async fn ensure_configmap(client: Client) -> Result<ConfigMap, DatabricksKubeError> {
    let cm_api = Api::<ConfigMap>::default_namespaced(client);

    let config_map = await_condition(
        cm_api,
        CONFIGMAP_NAME.as_str(),
        move |co: Option<&ConfigMap>| {
            if let Some(cmap) = co {
                if let Some(data) = &cmap.data {
                    if let (Some(token), Some(url)) =
                        (data.get("access_token"), data.get("databricks_url"))
                    {
                        return (!token.is_empty()) && (!url.is_empty());
                    }
                }
            }

            false
        },
    );

    log::info!("Waiting for settings in config map: {}", *CONFIGMAP_NAME);

    timeout(Duration::from_secs(15), config_map)
        .await
        .into_iter()
        .flatten()
        .last()
        .flatten()
        .ok_or(DatabricksKubeError::ConfigMapMissingError)
}

pub async fn ensure_crd(
    name: &str,
    client: Client,
) -> Result<CustomResourceDefinition, DatabricksKubeError> {
    let crd_api = Api::<CustomResourceDefinition>::all(client);

    let config_map = await_condition(crd_api, name, conditions::is_crd_established());

    log::info!("Waiting for CRD: {}", name);

    timeout(Duration::from_secs(15), config_map)
        .await
        .into_iter()
        .flatten()
        .last()
        .flatten()
        .ok_or(DatabricksKubeError::CRDMissingError(name.to_string()))
}
