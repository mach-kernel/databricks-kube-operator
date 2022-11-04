use futures::{StreamExt, TryStreamExt};
use k8s_openapi::{
    api::core::v1::ConfigMap,
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
use std::sync::Arc;
use std::{collections::BTreeMap, env, time::Duration};
use tokio::{task::JoinHandle, time::timeout};

use crate::error::DatabricksKubeError;

use flurry::HashMap;

lazy_static! {
    pub static ref CONFIGMAP_NAME: String =
        env::var("DATABRICKS_KUBE_CONFIGMAP").unwrap_or("databricks-kube-operator".to_owned());
}

#[derive(Clone)]
pub struct Context {
    pub client: Client,
    pub delete_watchers: Arc<HashMap<String, Box<JoinHandle<Result<(), DatabricksKubeError>>>>>,
    store: Arc<Store<ConfigMap>>,
}

impl Context {
    pub async fn get_configmap_key(&self, key: &str) -> Option<String> {
        self.latest_config()
            .await
            .into_iter()
            .flat_map(|c| c.get(key).map(String::clone))
            .next()
    }

    pub async fn get_databricks_url_token(&self) -> Option<(String, String)> {
        let url = self.get_configmap_key("databricks_url").await?;
        let token = self.get_configmap_key("access_token").await?;
        Some((url, token))
    }

    pub async fn latest_config(&self) -> Option<BTreeMap<String, String>> {
        self.store
            .state()
            .into_iter()
            .flat_map(|x| x.data.clone())
            .next()
    }

    pub async fn new(client: Client) -> Result<Arc<Context>, DatabricksKubeError> {
        let cm_api = Api::<ConfigMap>::default_namespaced(client.clone());
        let crd_api = Api::<CustomResourceDefinition>::all(client.clone());

        Self::ensure_crd("databricksjobs.com.dstancu.databricks", crd_api.clone()).await?;
        Self::ensure_crd("gitcredentials.com.dstancu.databricks", crd_api).await?;
        Self::ensure_configmap(cm_api.clone()).await?;

        let store = Self::watch_configmap(cm_api).await?;

        Ok(Self {
            client,
            store,
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
        let mut rf = reflector::reflector(writer, watcher::watcher(cm_api, params)).boxed();

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
}
