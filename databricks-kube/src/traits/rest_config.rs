use std::{pin::Pin, time::Duration};

use crate::context::Context;

use databricks_rust_jobs::{apis::configuration::Configuration as JobClientConfig, models::Job};
use futures::FutureExt;

use databricks_rust_git_credentials::{
    apis::configuration::Configuration as GitCredentialClientConfig,
    models::GetCredentialResponse as GitCredential,
};
use tokio::time::interval;

use std::sync::Arc;

pub trait RestConfig<TConfigType> {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<TConfigType>> + std::marker::Send>>;
}

impl RestConfig<JobClientConfig> for Job {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<JobClientConfig>> + std::marker::Send>> {
        let mut wait = interval(Duration::from_secs(15));

        async move {
            while let None = context.get_databricks_url_token().await {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let (url, token) = context.get_databricks_url_token().await?;
            Some(JobClientConfig {
                base_path: url,
                bearer_access_token: Some(token),
                ..JobClientConfig::default()
            })
        }
        .boxed()
    }
}

impl RestConfig<GitCredentialClientConfig> for GitCredential {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<GitCredentialClientConfig>> + std::marker::Send>>
    {
        let mut wait = interval(Duration::from_secs(15));

        async move {
            while let None = context.get_databricks_url_token().await {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let (url, token) = context.get_databricks_url_token().await?;
            Some(GitCredentialClientConfig {
                base_path: url,
                bearer_access_token: Some(token),
                ..GitCredentialClientConfig::default()
            })
        }
        .boxed()
    }
}
