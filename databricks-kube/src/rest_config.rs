use std::pin::Pin;

use crate::context::Context;

use futures::FutureExt;
use databricks_rust_jobs::{apis::configuration::Configuration as JobClientConfig, models::Job};


use databricks_rust_git_credentials::{
    apis::{configuration::Configuration as GitCredentialClientConfig},
    models::GetCredentialResponse as GitCredential
};

pub trait RestConfig<TConfigType> {
    fn get_rest_config(context: Context) -> Pin<Box<dyn futures::Future<Output = Option<TConfigType>> + std::marker::Send>>;
}

impl RestConfig<JobClientConfig> for Job {
    fn get_rest_config(context: Context) -> Pin<Box<dyn futures::Future<Output = Option<JobClientConfig>> + std::marker::Send>> {
        async move {
            if let Some((url, token)) = context.get_databricks_url_token().await {
                Some(JobClientConfig {
                    base_path: url,
                    bearer_access_token: Some(token),
                    ..JobClientConfig::default()
                })
            } else {
                None
            }

        }.boxed()
    }
}

impl RestConfig<GitCredentialClientConfig> for GitCredential {
    fn get_rest_config(context: Context) -> Pin<Box<dyn futures::Future<Output = Option<GitCredentialClientConfig>> + std::marker::Send>> {
        async move {
            if let Some((url, token)) = context.get_databricks_url_token().await {
                Some(GitCredentialClientConfig {
                    base_path: url,
                    bearer_access_token: Some(token),
                    ..GitCredentialClientConfig::default()
                })
            } else {
                None
            }
        }.boxed()
    }
}