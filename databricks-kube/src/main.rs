mod context;
mod crds;
pub mod error;
pub mod traits;

use std::time::Duration;

use anyhow::Result;

use git_version::git_version;
use kube::Client;
use tokio_graceful_shutdown::{SubsystemHandle, Toplevel};

use crate::context::Context;
use crate::traits::synced_api_resource::SyncedAPIResource;
use crds::databricks_job::DatabricksJob;
use crds::git_credential::GitCredential;

use error::DatabricksKubeError;

// use controllers::databricks_job;

#[tokio::main]
async fn main() -> Result<(), DatabricksKubeError> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    let ctx = Context::new(kube_client.clone()).await?;

    let job_controller = DatabricksJob::controller(ctx.clone());
    let job_ingest = DatabricksJob::ingest_task(ctx.clone());

    let git_credential_controller = GitCredential::controller(ctx.clone());
    let git_credential_ingest = GitCredential::ingest_task(ctx.clone());

    Toplevel::new()
        .start(
            "job_controller",
            |_: SubsystemHandle<DatabricksKubeError>| job_controller,
        )
        .start("job_ingest", |_: SubsystemHandle<DatabricksKubeError>| {
            job_ingest
        })
        .start(
            "git_credential_controller",
            |_: SubsystemHandle<DatabricksKubeError>| git_credential_controller,
        )
        .start(
            "git_credential_ingest",
            |_: SubsystemHandle<DatabricksKubeError>| git_credential_ingest,
        )
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(5))
        .await
        .map_err(|gse| DatabricksKubeError::Shutdown(gse.to_string()))
}
