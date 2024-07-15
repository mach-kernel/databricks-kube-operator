mod context;
mod crds;
mod error;
mod traits;
mod util;

use std::{collections::BTreeMap, hash::Hash, sync::Arc, time::Duration};

use databricks_kube::{
    context::Context,
    crds::databricks_job::DatabricksJob,
    crds::databricks_secret_scope::DatabricksSecretScope,
    crds::git_credential::GitCredential,
    crds::repo::Repo,
    error::DatabricksKubeError,
    traits::{remote_api_resource::RemoteAPIResource, remote_api_status::RemoteAPIStatus},
    util::*,
};

use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};

use futures::FutureExt;
use futures::StreamExt;

use git_version::git_version;
use kube::{
    runtime::{
        controller::Action,
        reflector::{ObjectRef, Store},
    },
    Api, Client, CustomResourceExt, Resource, ResourceExt,
};
use tokio::time::sleep;
use tokio_graceful_shutdown::{SubsystemHandle, Toplevel};

fn latest_config(configmap_store: Arc<Store<ConfigMap>>) -> Option<BTreeMap<String, String>> {
    configmap_store
        .state()
        .into_iter()
        .flat_map(|x| x.data.clone())
        .next()
}

fn get_store_key(map: Option<BTreeMap<String, String>>, key: &str) -> Option<String> {
    map.into_iter().flat_map(|c| c.get(key).cloned()).next()
}

async fn log_controller_event<TCRDType>(
    event: Result<(ObjectRef<TCRDType>, Action), DatabricksKubeError>,
) where
    TCRDType: Resource + ResourceExt + CustomResourceExt,
    TCRDType::DynamicType: Default + Eq + Hash,
{
    match event {
        Ok((object, _)) => log::info!("{} reconciled", object.name),
        Err(e) => log::error!("{:?}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), DatabricksKubeError> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    log::info!("Client namespace {}", kube_client.default_namespace());

    let cm_api = Api::<ConfigMap>::default_namespaced(kube_client.clone());
    let crd_api = Api::<CustomResourceDefinition>::all(kube_client.clone());
    let secret_api = Api::<Secret>::default_namespaced(kube_client.clone());

    ensure_crd("databricksjobs.com.dstancu.databricks", crd_api.clone()).await?;
    ensure_crd("gitcredentials.com.dstancu.databricks", crd_api.clone()).await?;
    ensure_crd("repos.com.dstancu.databricks", crd_api.clone()).await?;
    ensure_crd("databrickssecretscopes.com.dstancu.databricks", crd_api).await?;
    ensure_configmap(cm_api.clone()).await?;

    let configmap_store = watch_configmap(cm_api.clone()).await?;

    while let None = get_store_key(latest_config(configmap_store.clone()), "api_secret_name") {
        sleep(Duration::from_millis(250)).await;
    }

    let api_secret_name = get_store_key(latest_config(configmap_store.clone()), "api_secret_name")
        .expect("Must define api_secret_name");

    ensure_api_secret(api_secret_name.clone(), secret_api.clone()).await?;
    let api_secret_store = watch_api_secret(api_secret_name.clone(), secret_api).await?;

    let ctx = Context::new(kube_client.clone(), api_secret_store, configmap_store);

    let job_controller = DatabricksJob::controller(ctx.clone());
    let job_status_controller = DatabricksJob::status_controller(ctx.clone());

    let git_credential_controller = GitCredential::controller(ctx.clone());
    let repo_controller = Repo::controller(ctx.clone());
    let secret_scope_controller = DatabricksSecretScope::controller(ctx.clone());

    Toplevel::new()
        .start(
            "job_controller",
            |_: SubsystemHandle<DatabricksKubeError>| {
                job_controller.for_each(log_controller_event).map(|_| {
                    let res: Result<(), DatabricksKubeError> = Ok(());
                    res
                })
            },
        )
        .start(
            "job_status_controller",
            |_: SubsystemHandle<DatabricksKubeError>| {
                job_status_controller
                    .for_each(log_controller_event)
                    .map(|_| {
                        let res: Result<(), DatabricksKubeError> = Ok(());
                        res
                    })
            },
        )
        .start(
            "git_credential_controller",
            |_: SubsystemHandle<DatabricksKubeError>| {
                git_credential_controller
                    .for_each(log_controller_event)
                    .map(|_| {
                        let res: Result<(), DatabricksKubeError> = Ok(());
                        res
                    })
            },
        )
        .start(
            "repo_controller",
            |_: SubsystemHandle<DatabricksKubeError>| {
                repo_controller.for_each(log_controller_event).map(|_| {
                    let res: Result<(), DatabricksKubeError> = Ok(());
                    res
                })
            },
        )
        .start(
            "secret_scope_controller",
            |_: SubsystemHandle<DatabricksKubeError>| {
                secret_scope_controller
                    .for_each(log_controller_event)
                    .map(|_| {
                        let res: Result<(), DatabricksKubeError> = Ok(());
                        res
                    })
            },
        )
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(1))
        .await
        .map_err(|gse| DatabricksKubeError::Shutdown(gse.to_string()))
}
