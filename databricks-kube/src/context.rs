use std::{collections::BTreeMap, env, sync::Arc};

use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    ByteString,
};
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

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema, Default)]
pub struct DatabricksAPISecret {
    pub databricks_url: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct OperatorConfiguration {
    pub api_secret_name: Option<String>,
    pub default_poll_interval: Option<u32>,
    pub default_timeout_seconds: Option<u32>,
    pub default_requeue_interval: Option<u64>,
}

impl Default for OperatorConfiguration {
    fn default() -> Self {
        Self {
            api_secret_name: None,
            default_poll_interval: Some(250),
            default_timeout_seconds: Some(10),
            default_requeue_interval: Some(300),
        }
    }
}

impl Context {
    pub fn get_operator_config(&self) -> Option<OperatorConfiguration> {
        let latest_config = Self::latest_config(self.configmap_store.clone())?;
        let defaults = OperatorConfiguration::default();

        Some(OperatorConfiguration {
            api_secret_name: latest_config.get("api_secret_name").cloned(),
            default_poll_interval: latest_config
                .get("default_poll_interval")
                .map(|v| v.parse::<u32>().unwrap())
                .or(defaults.default_poll_interval),
            default_timeout_seconds: latest_config
                .get("default_timeout_seconds")
                .map(|v| v.parse::<u32>().unwrap())
                .or(defaults.default_timeout_seconds),
            default_requeue_interval: latest_config
                .get("default_requeue_interval")
                .map(|v| v.parse::<u64>().unwrap())
                .or(defaults.default_requeue_interval),
        })
    }

    pub fn get_api_secret(&self) -> Option<DatabricksAPISecret> {
        let latest_secrets = Self::latest_store(self.api_secret_store.clone())?;

        Some(DatabricksAPISecret {
            databricks_url: latest_secrets.get("databricks_url").cloned(),
            access_token: latest_secrets.get("access_token").cloned(),
        })
    }

    fn latest_store(secret_store: Arc<Store<Secret>>) -> Option<BTreeMap<String, String>> {
        secret_store
            .state()
            .iter()
            .flat_map(|s| {
                s.data.as_ref().map(|m| {
                    BTreeMap::from_iter(m.iter().map(|(k, ByteString(v))| {
                        (k.clone(), String::from_utf8(v.clone()).unwrap())
                    }))
                })
            })
            .next()
    }

    fn latest_config(config_store: Arc<Store<ConfigMap>>) -> Option<BTreeMap<String, String>> {
        config_store
            .state()
            .iter()
            .flat_map(|x| x.data.clone())
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
