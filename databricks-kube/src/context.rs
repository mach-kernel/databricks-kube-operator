use std::{collections::BTreeMap, env, sync::Arc};

use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use kube::{runtime::reflector::Store, Client};
use lazy_static::lazy_static;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIGMAP_NAME: String =
        env::var("DATABRICKS_KUBE_CONFIGMAP").unwrap_or("databricks-kube-operator".to_owned());
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Context {
    pub client: Client,
    configmap_store: Arc<Store<ConfigMap>>,
    api_secret_store: Arc<Store<Secret>>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema, Debug, Default)]
pub struct DatabricksAPISecret {
    pub databricks_url: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct OperatorConfiguration {
    pub api_secret_name: Option<String>,
    pub default_poll_interval: Option<u32>,
    pub default_timeout_seconds: Option<u32>,
    pub default_requeue_interval: Option<u64>
}

impl Default for OperatorConfiguration {
    fn default() -> Self {
        Self {
            api_secret_name: Some(String::from("default_secret_name")),
            default_poll_interval: Some(250),
            default_timeout_seconds: Some(10),
            default_requeue_interval: Some(300)
        }
    }
}

impl Context {
    pub fn get_operator_config(&self) -> Option<OperatorConfiguration> {
        let latest_config = Self::latest_config(self.configmap_store.clone())?;

        let mut op_config = OperatorConfiguration::default();
        
        op_config.api_secret_name = Some(String::from(latest_config.get("api_secret_name").unwrap().to_string()));
        op_config.default_poll_interval = Some(latest_config.get("default_poll_interval")?.parse::<u32>().unwrap());
        op_config.default_timeout_seconds = Some(latest_config.get("default_timeout_seconds")?.parse::<u32>().unwrap());
        op_config.default_requeue_interval = Some(latest_config.get("default_requeue_interval")?.parse::<u64>().unwrap());

        Some(op_config)
    }

    pub fn get_api_secret(&self) -> Option<DatabricksAPISecret> {
        let latest_secret = Self::latest_store(self.api_secret_store.clone())?;

        let mut api_secret = DatabricksAPISecret::default();

        api_secret.databricks_url = Some(String::from(latest_secret.get("databricks_url").unwrap().to_string()));
        api_secret.access_token = Some(String::from(latest_secret.get("access_token").unwrap().to_string()));

        Some(api_secret)

    }
    

    fn latest_store(secret_store: Arc<Store<Secret>>) -> Option<BTreeMap<String, String>> {
        secret_store
            .state()
            .into_iter()
            .map(|x| {
                BTreeMap::from_iter(
                    x.data
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|(k, v)| (k, std::str::from_utf8(&v.0).unwrap().to_string())),
                )
            })
            .next()
    }

    fn latest_config(config_store: Arc<Store<ConfigMap>>) -> Option<BTreeMap<String, String>> {
        config_store
            .state()
            .into_iter()
            .map(|x| {
                BTreeMap::from_iter(
                    x.data
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|(k, v)| (k, v)),
                )
            })
            .next()
    }

    #[allow(dead_code)]
    pub fn new(
        client: Client,
        api_secret_store: Arc<Store<Secret>>,
        configmap_store: Arc<Store<ConfigMap>>,
    ) -> Arc<Context> {
        Self {
            api_secret_store,
            client,
            configmap_store,
        }
        .into()
    }
}
