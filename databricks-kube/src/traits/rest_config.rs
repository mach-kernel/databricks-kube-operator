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

use databricks_rust_secrets::{
    apis::configuration::Configuration as SecretClientConfig,
    models::WorkspaceGetSecretResponse as Secret, models::WorkspaceListScopesResponse as Scopes,
    models::WorkspaceSecretScope as Scope,
};

use futures::FutureExt;
use tokio::time::interval;

pub trait RestConfig<TConfigType> {
    fn get_rest_config(
        context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Option<TConfigType>> + std::marker::Send>>;
}

#[macro_export]
macro_rules! openapi_config_glue {
    ($client_config:ident, $dto:ident) => {
        impl RestConfig<$client_config> for $dto {
            fn get_rest_config(
                context: Arc<Context>,
            ) -> Pin<Box<dyn futures::Future<Output = Option<$client_config>> + std::marker::Send>>
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

                    Some($client_config {
                        base_path: url,
                        bearer_access_token: Some(token),
                        ..$client_config::default()
                    })
                }
                .boxed()
            }
        }
    };
}

openapi_config_glue!(JobClientConfig, Job);
openapi_config_glue!(GitCredentialClientConfig, GitCredential);
openapi_config_glue!(RepoClientConfig, Repo);
openapi_config_glue!(SecretClientConfig, Secret);
openapi_config_glue!(SecretClientConfig, Scope);
openapi_config_glue!(SecretClientConfig, Scopes);
