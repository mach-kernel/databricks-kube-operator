mod config;
mod crd;
mod error;

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use git_version::git_version;
use kube::Client;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    let cfg = Config::new(kube_client).await?;

    sleep(Duration::from_secs(1));
    println!(
        "get url {:?}",
        cfg.get_configmap_key("databricks_url").await
    );

    Ok(())
}
