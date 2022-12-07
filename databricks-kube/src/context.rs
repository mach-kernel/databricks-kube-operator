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

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct DatabricksAPISecret {
    pub databricks_url: String,
    pub access_token: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct OperatorConfiguration {
    pub api_secret_name: String,
}

impl Context {
    pub fn get_databricks_url_token(&self) -> Option<(String, String)> {
        let latest_secret = Self::latest_api_secret(self.api_secret_store.clone())?;

        let url = latest_secret.get("databricks_url")?;
        let token = latest_secret.get("access_token")?;
        Some((url.to_string(), token.to_string()))
    }

    fn latest_api_secret(api_secret_store: Arc<Store<Secret>>) -> Option<BTreeMap<String, String>> {
        api_secret_store
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
