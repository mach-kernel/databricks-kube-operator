mod context;
mod crds;
pub mod error;
pub mod traits;

use std::time::Duration;

use anyhow::Result;

use futures::{future::join_all, FutureExt};
use git_version::git_version;
use kube::Client;

use crate::context::Context;
use crate::traits::synced_api_resource::SyncedAPIResource;
use crds::databricks_job::DatabricksJob;

// use controllers::databricks_job;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    let ctx = Context::new(kube_client.clone()).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    join_all(vec![
        DatabricksJob::spawn_remote_ingest_task(ctx.clone()),
        DatabricksJob::spawn_controller(ctx.clone()),
    ])
    .await;

    Ok(())
}
