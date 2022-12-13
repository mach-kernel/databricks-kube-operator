use std::sync::Arc;
use std::{pin::Pin, time::Duration};

use crate::context::Context;

use databricks_rust_git_credentials::{
    apis::configuration::Configuration as GitCredentialClientConfig,
    models::GetCredentialResponse as GitCredential,
};
use databricks_rust_jobs::{apis::configuration::Configuration as JobClientConfig, models::Job};
use databricks_rust_repos::{
    apis::configuration::Configuration as RepoClientConfig, models::GetRepoResponse as Repo,
};

use futures::FutureExt;
use tokio::time::interval;

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
            while let None = context.get_databricks_url_token() {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let opConfig = context.get_databricks_url_token()?;
            let (url, token) = (opConfig.get("databricks_url").unwrap().to_string(), opConfig.get("access_token").unwrap().to_string());
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
            while let None = context.get_databricks_url_token() {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let opConfig = context.get_databricks_url_token()?;
            let (url, token) = (opConfig.get("databricks_url").unwrap().to_string(), opConfig.get("access_token").unwrap().to_string());
            Some(GitCredentialClientConfig {
                base_path: format!("{}/2.0", url),
                bearer_access_token: Some(token),
                ..GitCredentialClientConfig::default()
            })
        }
        .boxed()
    }
}

impl RestConfig<RepoClientConfig> for Repo {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<RepoClientConfig>> + std::marker::Send>> {
        let mut wait = interval(Duration::from_secs(15));

        async move {
            while let None = context.get_databricks_url_token() {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let opConfig = context.get_databricks_url_token()?;
            let (url, token) = (opConfig.get("databricks_url").unwrap().to_string(), opConfig.get("access_token").unwrap().to_string());
            Some(RepoClientConfig {
                base_path: format!("{}/2.0", url),
                bearer_access_token: Some(token),
                ..RepoClientConfig::default()
            })
        }
        .boxed()
    }
}
