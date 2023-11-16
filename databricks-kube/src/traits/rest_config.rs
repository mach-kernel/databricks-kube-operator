use std::sync::Arc;
use std::{pin::Pin, time::Duration};

use crate::context::Context;
use databricks_rust_openapi::apis::configuration::Configuration;

use databricks_rust_openapi::models::{JobsJob, WorkspaceCredentialInfo, WorkspaceRepoInfo};
use futures::FutureExt;
use tokio::time::interval;

pub trait RestConfig<TConfigType> {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<TConfigType>> + std::marker::Send>>;
}

impl RestConfig<Configuration> for JobsJob {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<Configuration>> + std::marker::Send>> {
        let mut wait = interval(Duration::from_secs(15));

        async move {
            while let None = context.get_api_secret() {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let api_secret = context.get_api_secret()?;
            let (url, token) = (
                api_secret.databricks_url.unwrap().to_string(),
                api_secret.access_token.unwrap().to_string(),
            );
            Some(Configuration {
                base_path: url,
                bearer_access_token: Some(token),
                ..Configuration::default()
            })
        }
        .boxed()
    }
}

impl RestConfig<Configuration> for WorkspaceCredentialInfo {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<Configuration>> + std::marker::Send>>
    {
        let mut wait = interval(Duration::from_secs(15));

        async move {
            while let None = context.get_api_secret() {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let api_secret = context.get_api_secret()?;
            let (url, token) = (
                api_secret.databricks_url.unwrap().to_string(),
                api_secret.access_token.unwrap().to_string(),
            );
            Some(Configuration {
                base_path: format!("{}/2.0", url),
                bearer_access_token: Some(token),
                ..Configuration::default()
            })
        }
        .boxed()
    }
}

impl RestConfig<Configuration> for WorkspaceRepoInfo {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<Configuration>> + std::marker::Send>> {
        let mut wait = interval(Duration::from_secs(15));

        async move {
            while let None = context.get_api_secret() {
                wait.tick().await;
                log::info!("Waiting for REST credentials...");
            }

            let api_secret = context.get_api_secret()?;
            let (url, token) = (
                api_secret.databricks_url.unwrap().to_string(),
                api_secret.access_token.unwrap().to_string(),
            );
            Some(Configuration {
                base_path: format!("{}/2.0", url),
                bearer_access_token: Some(token),
                ..Configuration::default()
            })
        }
        .boxed()
    }
}
