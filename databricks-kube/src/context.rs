use std::{collections::BTreeMap, env, sync::Arc, time::Duration};

use crate::error::DatabricksKubeError;

use flurry::HashMap;
use futures::{StreamExt, TryStreamExt};
use jsonschema::is_valid;
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    api::ListParams,
    runtime::{
        reflector::{self, Store},
        wait::{await_condition, conditions},
        watcher::{self, Event},
    },
    Api, Client,
};
use lazy_static::lazy_static;
use schemars::{gen::SchemaGenerator, schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{task::JoinHandle, time::{timeout, sleep}};

lazy_static! {
    pub static ref CONFIGMAP_NAME: String =
        env::var("DATABRICKS_KUBE_CONFIGMAP").unwrap_or("databricks-kube-operator".to_owned());
}

#[derive(Clone)]
pub struct Context {
    pub client: Client,
    pub delete_watchers: Arc<HashMap<String, Box<JoinHandle<Result<(), DatabricksKubeError>>>>>,
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
    fn get_store_key(map: Option<BTreeMap<String, String>>, key: &str) -> Option<String> {
        map
            .into_iter()
            .flat_map(|c| c.get(key).map(String::clone))
            .next()
    }

    pub fn get_databricks_url_token(&self) -> Option<(String, String)> {
        let latest_secret = Self::latest_api_secret(self.api_secret_store.clone())?;

        let url = latest_secret.get("databricks_url")?;
        let token = latest_secret.get("access_token")?;
        Some((url.to_string(), token.to_string()))
    }

    fn latest_config(configmap_store: Arc<Store<ConfigMap>>) -> Option<BTreeMap<String, String>> {
        configmap_store
            .state()
            .into_iter()
            .flat_map(|x| x.data.clone())
            .next()
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
    pub async fn new(client: Client) -> Result<Arc<Context>, DatabricksKubeError> {
        let cm_api = Api::<ConfigMap>::default_namespaced(client.clone());
        let crd_api = Api::<CustomResourceDefinition>::all(client.clone());
        let secret_api = Api::<Secret>::all(client.clone());

        Self::ensure_crd("databricksjobs.com.dstancu.databricks", crd_api.clone()).await?;
        Self::ensure_crd("gitcredentials.com.dstancu.databricks", crd_api.clone()).await?;
        Self::ensure_crd("repos.com.dstancu.databricks", crd_api).await?;
        Self::ensure_configmap(cm_api.clone()).await?;

        let configmap_store = Self::watch_configmap(cm_api.clone()).await?;


        while let None = Self::get_store_key(Self::latest_config(configmap_store.clone()), "api_secret_name") {
            sleep(Duration::from_millis(250)).await;
        }

        let api_secret_name = Self::get_store_key(Self::latest_config(configmap_store.clone()), "api_secret_name").unwrap();

        Self::ensure_api_secret(api_secret_name.clone(), secret_api.clone()).await?;
        let api_secret_store = Self::watch_api_secret(api_secret_name.clone(), secret_api).await?;

        Ok(Self {
            api_secret_store,
            client,
            configmap_store,
            delete_watchers: HashMap::new().into(),
        }
        .into())
    }

    async fn watch_configmap(
        cm_api: Api<ConfigMap>,
    ) -> Result<Arc<Store<ConfigMap>>, DatabricksKubeError> {
        let params = ListParams {
            field_selector: Some(format!("metadata.name={}", *CONFIGMAP_NAME)),
            ..ListParams::default()
        };

        // Make a new reflector store, pin it
        let (reader, writer) = reflector::store();
        let mut rf = reflector::reflector(writer, watcher::watcher(cm_api.clone(), params)).boxed();

        // Spawn tokio orphan, die if no conf
        tokio::spawn(async move {
            while let Ok(maybe_event) = rf.try_next().await {
                if let Some(event) = maybe_event {
                    match event {
                        Event::Deleted(cm) => {
                            log::error!(
                                "Config map {} was deleted, exiting",
                                cm.metadata.name.unwrap()
                            );
                            panic!("Config map deleted");
                        }
                        Event::Applied(cm) => {
                            Self::ensure_configmap(cm_api.clone()).await.unwrap();
                            log::info!("Config map {} applied", cm.metadata.name.unwrap());
                        }
                        Event::Restarted(v) => {
                            log::info!(
                                "Watching config map {}",
                                v.get(0).unwrap().metadata.name.clone().unwrap()
                            )
                        }
                    };
                };
            }
        });

        Ok(Arc::new(reader))
    }

    async fn watch_api_secret(
        api_secret_name: String,
        secret_api: Api<Secret>,
    ) -> Result<Arc<Store<Secret>>, DatabricksKubeError> {
        let params = ListParams {
            field_selector: Some(format!("metadata.name={}", api_secret_name.clone())),
            ..ListParams::default()
        };

        // Make a new reflector store, pin it
        let (reader, writer) = reflector::store();
        let mut rf =
            reflector::reflector(writer, watcher::watcher(secret_api.clone(), params)).boxed();

        // Spawn tokio orphan, die if no conf
        tokio::spawn(async move {
            while let Ok(maybe_event) = rf.try_next().await {
                if let Some(event) = maybe_event {
                    match event {
                        Event::Deleted(cm) => {
                            log::error!(
                                "Secret {} was deleted, exiting",
                                cm.metadata.name.unwrap()
                            );
                            panic!("Secret deleted");
                        }
                        Event::Applied(cm) => {
                            Self::ensure_api_secret(api_secret_name.clone(), secret_api.clone())
                                .await.unwrap();
                            log::info!("Secret {} applied", cm.metadata.name.unwrap());
                        }
                        Event::Restarted(v) => {
                            log::info!(
                                "Watching secret {}",
                                v.get(0).unwrap().metadata.name.clone().unwrap()
                            )
                        }
                    };
                };
            }
        });

        Ok(Arc::new(reader))
    }

    async fn ensure_crd(
        name: &str,
        crd_api: Api<CustomResourceDefinition>,
    ) -> Result<CustomResourceDefinition, DatabricksKubeError> {
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

    async fn ensure_configmap(cm_api: Api<ConfigMap>) -> Result<ConfigMap, DatabricksKubeError> {
        let config_map = await_condition(
            cm_api.clone(),
            CONFIGMAP_NAME.as_str(),
            move |co: Option<&ConfigMap>| {
                if let Some(cmap) = co {
                    if let Some(data) = &cmap.data {
                        let schema = json!(schema_for!(OperatorConfiguration));

                        let mut val_map = serde_json::Map::new();
                        val_map.extend(
                            data.iter()
                                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone()))),
                        );

                        let valid = is_valid(&schema, &serde_json::Value::Object(val_map));
                        if !valid {
                            log::error!(
                                "Configmap {} does not have a valid schema.",
                                *CONFIGMAP_NAME
                            );
                        }

                        return valid;
                    }
                }
                false
            },
        );

        log::info!("Waiting for settings in config map: {}", *CONFIGMAP_NAME);

        let found = timeout(Duration::from_secs(15), config_map)
            .await
            .into_iter()
            .flatten()
            .last()
            .flatten()
            .ok_or(DatabricksKubeError::ConfigMapMissingError)?;

        log::info!("Found config map");

        Ok(found)
    }

    async fn ensure_api_secret(
        api_secret_name: String,
        secret_api: Api<Secret>,
    ) -> Result<Secret, DatabricksKubeError> {
        let api_secret = await_condition(
            secret_api.clone(),
            &api_secret_name,
            |co: Option<&Secret>| {
                if let Some(secret) = co {
                    if let Some(data) = &secret.data {
                        let schema = json!(schema_for!(DatabricksAPISecret));

                        let mut val_map = serde_json::Map::new();
                        val_map.extend(data.iter().map(|(k, v)| {
                            (
                                k.clone(),
                                serde_json::Value::String(
                                    std::str::from_utf8(&v.0).unwrap().to_string(),
                                ),
                            )
                        }));

                        let valid = is_valid(&schema, &serde_json::Value::Object(val_map));
                        if !valid {
                            log::error!("Secret {} does not have a valid schema.", api_secret_name);
                        }

                        return valid;
                    }
                }
                false
            },
        );

        log::info!("Waiting for API secret: {}", api_secret_name);

        let found = timeout(Duration::from_secs(15), api_secret)
            .await
            .into_iter()
            .flatten()
            .last()
            .flatten()
            .ok_or(DatabricksKubeError::ConfigMapMissingError)?;

        log::info!("Found API secret");

        Ok(found)
    }
}
