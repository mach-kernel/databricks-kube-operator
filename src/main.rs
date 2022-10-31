mod config;
mod crd;
mod error;

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use git_version::git_version;
use kube::{Api, Client, ResourceExt, api::PostParams};
use tokio::time::interval;

use crate::config::Config;
use crd::databricks_job::{DatabricksJob, DatabricksJobSpec};
use databricks_rust_jobs::{
    apis::{configuration::Configuration, default_api},
    models::JobsList200Response,
};


// TODO: yes this is awful, there needs to be a reconciler, but this is the
// cheapest possible demo!!! yay!!
async fn poll_databricks_jobs(config: Config, kube_jobs: Api<DatabricksJob>) {
    let mut duration = interval(Duration::from_secs(10));

    loop {
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

            if let Some(jobs) = default_api::jobs_list(&databricks_config, None, None, None)
                .await
                .ok()
                .into_iter()
                .flat_map(|r| r.jobs)
                .next() {
                    for job in jobs {
                        let name = job.clone().settings.unwrap().name.unwrap();

                        let kube_job = kube_jobs.get(&name).await;
                        if kube_job.is_err() {
                            log::info!("Job {} missing in K8S, creating DatabricksJob", name);
                            let new_job = DatabricksJob::new(&name, DatabricksJobSpec {
                                job,
                            });

                            if let Ok(j) = kube_jobs.create(&PostParams::default(), &new_job).await {
                                log::info!("Created DatabricksJob {}", j.metadata.name.unwrap());
                            };

                        }
                    }
                }
        }

        duration.tick().await;
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
