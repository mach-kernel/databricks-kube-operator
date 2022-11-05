mod context;
mod crds;
mod error;
mod traits;

use std::time::Duration;

use context::Context;
use crds::databricks_job::DatabricksJob;
use crds::git_credential::GitCredential;
use crds::repo::Repo;
use error::DatabricksKubeError;
use traits::synced_api_resource::SyncedAPIResource;

use git_version::git_version;
use kube::Client;
use tokio_graceful_shutdown::{SubsystemHandle, Toplevel};

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

    let repo_controller = Repo::controller(ctx.clone());
    let repo_ingest = Repo::ingest_task(ctx.clone());

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
        .start(
            "repo_controller",
            |_: SubsystemHandle<DatabricksKubeError>| repo_controller,
        )
        .start("repo_ingest", |_: SubsystemHandle<DatabricksKubeError>| {
            repo_ingest
        })
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(1))
        .await
        .map_err(|gse| DatabricksKubeError::Shutdown(gse.to_string()))
}
