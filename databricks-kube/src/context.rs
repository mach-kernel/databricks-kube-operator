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

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema, Debug)]
pub struct DatabricksAPISecret {
    pub databricks_url: String,
    pub access_token: String,
}

impl Default for DatabricksAPISecret {
    fn default() -> Self {
        Self {
            databricks_url: String::from("default_url"),
            access_token: String::from("default_token"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct OperatorConfiguration {
    pub api_secret_name: String,
    pub default_poll_interval: String,
    pub default_timeout_seconds: String,
    pub default_requeue_interval: String,
    pub secrets: Option<BTreeMap<String, String>>
}

impl Default for OperatorConfiguration {
    fn default() -> Self {
        Self {
            api_secret_name: String::from("default_secret_name"),
            default_poll_interval: String::from("250"),
            default_timeout_seconds: String::from("10"),
            default_requeue_interval: String::from("300"),
            secrets: Some(BTreeMap::new())
        }
    }
}

impl Context {
    pub fn get_databricks_url_token(&self) -> Option<BTreeMap<String, String>> {
        let latest_secret = Self::latest_store(self.api_secret_store.clone())?;

        let options = OperatorConfiguration::default();

        let url = latest_secret.get("databricks_url")?;
        let token = latest_secret.get("access_token")?;

        let mut secret_map = options.secrets.unwrap();
        secret_map.insert(String::from("databricks_url"), String::from(url.to_string()));
        secret_map.insert(String::from("access_token"), String::from(token.to_string()));
        Some(secret_map)
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
