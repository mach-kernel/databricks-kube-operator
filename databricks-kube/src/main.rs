mod config;
mod crds;
pub mod error;
pub mod traits;

use anyhow::Result;
use futures::stream;
use futures::{StreamExt, TryStreamExt};
use git_version::git_version;
use kube::{api::PostParams, Api, Client, ResourceExt};
use std::{thread::sleep, time::Duration};
use tokio::time::interval;

use crate::config::Config;
use crds::databricks_job::{DatabricksJob, DatabricksJobSpec};
use databricks_rust_jobs::{
    apis::{configuration::Configuration, default_api},
    models::JobsList200Response,
};

use crate::traits::remote_resource::RemoteResource;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    let cfg = Config::new(kube_client.clone()).await?;

    let kube_jobs = Api::<DatabricksJob>::default_namespaced(kube_client);

    DatabricksJob::task_sync_remote_to_kube(cfg.clone()).await?;

    Ok(())
}
