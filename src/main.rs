mod config;
mod crd;
mod error;

use anyhow::Result;
use git_version::git_version;
use kube::Client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    config::ensure_crd("databricksjobs.com.dstancu", kube_client.clone()).await?;
    config::ensure_configmap(kube_client.clone()).await?;

    Ok(())
}
