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

// TODO: yes this is awful, there needs to be a reconciler, but this is the
// cheapest possible demo!!! yay!!
async fn poll_databricks_jobs(config: Config, kube_jobs: Api<DatabricksJob>) {
    let mut duration = interval(Duration::from_secs(10));

    loop {
        duration.tick().await;

        if let (Some(url), Some(token)) = (
            config.get_configmap_key("databricks_url").await,
            config.get_configmap_key("access_token").await,
        ) {
            log::info!("Polling Databricks for jobs!");

            let databricks_config = Configuration {
                base_path: url,
                bearer_access_token: Some(token),
                ..Configuration::default()
            };

            let mut jobs_stream = DatabricksJob::list_all(databricks_config);

            while let Ok(Some(job)) = jobs_stream.try_next().await {
                let job_as_kube: DatabricksJob = job.into();
                let name = job_as_kube.metadata.name.clone().unwrap();

                let kube_job = kube_jobs.get(&name).await;
                if kube_job.is_err() {
                    log::info!("Job {} missing in K8S, creating DatabricksJob", name);

                    if let Ok(j) = kube_jobs.create(&PostParams::default(), &job_as_kube).await {
                        log::info!("Created DatabricksJob {}", j.metadata.name.unwrap());
                    };
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("boot! (build: {})", git_version!());

    let kube_client = Client::try_default().await.expect("Must create client");
    let cfg = Config::new(kube_client.clone()).await?;

    let kube_jobs = Api::<DatabricksJob>::default_namespaced(kube_client);
    tokio::spawn(poll_databricks_jobs(cfg, kube_jobs)).await?;

    Ok(())
}
