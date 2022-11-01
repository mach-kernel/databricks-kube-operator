mod context;
mod crds;
mod controllers;
pub mod error;
pub mod traits;

use std::time::Duration;

use anyhow::Result;

use futures::{future::join_all, FutureExt};
use git_version::git_version;
use kube::{Client};

use crate::{context::Context};
use crds::databricks_job::DatabricksJob;

use crate::traits::remote_resource::RemoteResource;
use controllers::databricks_job;


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    let ctx = Context::new(kube_client.clone()).await?;

    log::info!("Waiting 10s to select init tasks");
    tokio::time::sleep(Duration::from_secs(10)).await;

    join_all(vec![
        DatabricksJob::spawn_remote_sync_task(ctx.clone()),
        databricks_job::spawn_controller(ctx.clone()).boxed(),
    ]).await;

    Ok(())
}
