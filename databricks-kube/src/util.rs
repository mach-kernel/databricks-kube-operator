#![allow(dead_code)]

use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;

use std::hash::Hash;

use std::{sync::Arc, time::Duration};

use crate::context::CONFIGMAP_NAME;
use crate::context::{DatabricksAPISecret, OperatorConfiguration};
use crate::error::DatabricksKubeError;

use futures::{StreamExt, TryStreamExt};
use jsonschema::is_valid;
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    runtime::{
        reflector::{self, Store},
        wait::{await_condition, conditions},
        watcher::{self, Event},
    },
    Api,
};

use schemars::schema_for;
use tokio::time::timeout;

pub async fn watch_api_secret(
    api_secret_name: String,
    secret_api: Api<Secret>,
) -> Result<Arc<Store<Secret>>, DatabricksKubeError> {
    let params = watcher::Config {
        field_selector: Some(format!("metadata.name={}", api_secret_name.clone())),
        ..watcher::Config::default()
    };

    // Make a new reflector store, pin it
    let (reader, writer) = reflector::store();
    let mut rf = reflector::reflector(writer, watcher::watcher(secret_api.clone(), params)).boxed();

    // Spawn tokio orphan, die if no conf
    tokio::spawn(async move {
        while let Ok(maybe_event) = rf.try_next().await {
            if let Some(event) = maybe_event {
                match event {
                    Event::Deleted(cm) => {
                        log::error!("Secret {} was deleted, exiting", cm.metadata.name.unwrap());
                        panic!("Secret deleted");
                    }
                    Event::Applied(cm) => {
                        ensure_api_secret(api_secret_name.clone(), secret_api.clone())
                            .await
                            .unwrap();
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

pub async fn watch_configmap(
    cm_api: Api<ConfigMap>,
) -> Result<Arc<Store<ConfigMap>>, DatabricksKubeError> {
    let params = watcher::Config {
        field_selector: Some(format!("metadata.name={}", *CONFIGMAP_NAME)),
        ..watcher::Config::default()
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
                        ensure_configmap(cm_api.clone()).await.unwrap();
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

pub async fn ensure_api_secret(
    api_secret_name: String,
    secret_api: Api<Secret>,
) -> Result<Secret, DatabricksKubeError> {
    let api_secret = await_condition(
        secret_api.clone(),
        &api_secret_name,
        |co: Option<&Secret>| {
            if let Some(data) = co.and_then(|cm| cm.data.clone()) {
                let schema = json!(schema_for!(DatabricksAPISecret));
                let json_data = json!(data);

                let valid = is_valid(&schema, &json_data);
                if !valid {
                    log::error!("Secret {} does not have a valid schema.", api_secret_name);
                }

                return valid;
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

pub async fn ensure_configmap(cm_api: Api<ConfigMap>) -> Result<ConfigMap, DatabricksKubeError> {
    let config_map = await_condition(
        cm_api.clone(),
        CONFIGMAP_NAME.as_str(),
        move |co: Option<&ConfigMap>| {
            if let Some(data) = co.and_then(|cm| cm.data.clone()) {
                let schema = json!(schema_for!(OperatorConfiguration));
                let json_data = json!(data);

                let valid = is_valid(&schema, &json_data);
                if !valid {
                    log::error!(
                        "Configmap {} does not have a valid schema.",
                        *CONFIGMAP_NAME
                    );
                }
                return valid;
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

pub async fn ensure_crd(
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

pub fn hash_json_value(hash: &mut DefaultHasher, value: &Value) {
    match value {
        Value::Null => 0.hash(hash),
        Value::Bool(b) => b.hash(hash),
        Value::String(s) => s.hash(hash),
        Value::Number(n) => n.hash(hash),
        Value::Array(a) => {
            for v in a {
                hash_json_value(hash, v)
            }
        }
        Value::Object(o) => {
            let mut keys: Vec<String> = o.keys().cloned().collect();
            keys.sort();

            for k in keys {
                hash_json_value(hash, o.get(&k).unwrap())
            }
        }
    }
}
