mod config;
mod crd;
mod error;

use databricks_rust_jobs::apis::configuration::Configuration;
use databricks_rust_jobs::apis::default_api::jobs_list;
use env_logger::Env;
use kube::{Client};
use git_version::git_version;
use crate::error::DatabricksKubeError;


#[tokio::main]
async fn main() -> Result<(), DatabricksKubeError> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());
    
    let kube_client = Client::try_default().await.expect("Must create client");
    let db_kube_configmap = config::ensure_configmap(kube_client.clone()).await?;

    Ok(())
}
