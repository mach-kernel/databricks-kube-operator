use std::sync::Arc;
use std::{pin::Pin, time::SystemTime};

use crate::context::Context;
use crate::traits::rest_config::RestConfig;
use crate::{error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource};


use async_stream::try_stream;
use databricks_rust_openapi::apis::git_credentials_api;
use databricks_rust_openapi::models::{WorkspaceCredentialInfo, WorkspaceGetCredentialsResponse, WorkspaceCreateCredentials, WorkspaceCreateCredentialsResponse, WorkspaceUpdateCredentials};
use futures::{Stream, StreamExt};
use k8s_openapi::{
    api::core::v1::Secret,
    serde::{Deserialize, Serialize},
};
use kube::{core::object::HasSpec, Api, CustomResource};

use schemars::JsonSchema;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "GitCredential",
    derive = "Default",
    namespaced
)]
pub struct GitCredentialSpec {
    pub credential: WorkspaceCredentialInfo,
    // The user provides an API token during a create request, but it
    // is otherwise no longer retrievable. Even with Helm/GitOps workflow,
    // the secret doesn't have to be checked in and could come from AWS
    // SSM or secrets manager.
    pub secret_name: Option<String>,
}

/// API -> CRD
impl From<WorkspaceCredentialInfo> for GitCredential {
    fn from(credential: WorkspaceCredentialInfo) -> Self {
        let credential_name = if let Some(cid) = &credential.credential_id {
            cid.to_string()
        } else if let Some(git_username) = &credential.git_username {
            git_username.clone()
        } else {
            format!(
                "noname-{}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            )
        };

        Self::new(
            &credential_name,
            GitCredentialSpec {
                credential,
                secret_name: None,
            },
        )
    }
}

/// CRD -> API
impl From<GitCredential> for WorkspaceCredentialInfo {
    fn from(value: GitCredential) -> Self {
        value.spec().credential.clone()
    }
}

impl RemoteAPIResource<WorkspaceCredentialInfo> for GitCredential {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<WorkspaceCredentialInfo, DatabricksKubeError>> + Send>> {
        try_stream! {
            let config = WorkspaceCredentialInfo::get_rest_config(context.clone()).await.unwrap();

            while let WorkspaceGetCredentialsResponse {
                credentials,
                ..
            } = git_credentials_api::git_credentialslist(&config).await? {
                if let Some(credentials) = credentials {
                    for credential in credentials {
                        yield credential;
                    }
                }
            }
        }
        .boxed()
    }

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<WorkspaceCredentialInfo, DatabricksKubeError>> + Send>> {
        let credential_id =
            self.spec()
                .credential
                .credential_id
                .ok_or(DatabricksKubeError::APIError(
                    "Remote resource cannot exist".to_string(),
                ));

        try_stream! {
            let config = WorkspaceCredentialInfo::get_rest_config(context.clone()).await.unwrap();
            let res = git_credentials_api::git_credentialsget(&config, credential_id?).await?;
            yield res
        }
        .boxed()
    }

    fn remote_create(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>> {
        let credential = self.spec().credential.clone();

        try_stream! {
            let config = WorkspaceCredentialInfo::get_rest_config(context.clone()).await.unwrap();

            let secret_name = self.spec().secret_name.clone().ok_or(DatabricksKubeError::SecretMissingError)?;
            log::info!("Reading secret {}", secret_name);

            let secrets_api = Api::<Secret>::default_namespaced(context.client.clone());
            let personal_access_token = secrets_api
                .get(&secret_name)
                .await
                .iter()
                .flat_map(|s| s.data.clone())
                .flat_map(|m| m.get("personal_access_token").map(Clone::clone))
                .flat_map(|buf| std::str::from_utf8(&buf.0).ok().map(ToString::to_string))
                .next()
                .ok_or(DatabricksKubeError::SecretMissingError)?;

            let WorkspaceCreateCredentialsResponse {
                credential_id,
                git_provider,
                git_username
            } = git_credentials_api::git_credentialscreate(
                &config,
                WorkspaceCreateCredentials {
                    personal_access_token: Some(personal_access_token),
                    git_username: credential.git_username,
                    git_provider: credential.git_provider.unwrap(),
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.credential = WorkspaceCredentialInfo {
                credential_id,
                git_provider,
                git_username
            };
            yield with_response;
        }.boxed()
    }

    fn remote_update(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let credential = self.spec().credential.clone();
        let credential_id = self.spec().credential.credential_id;

        try_stream! {
            let config = WorkspaceCredentialInfo::get_rest_config(context.clone()).await.unwrap();

            let secret_name = self.spec().secret_name.clone().ok_or(DatabricksKubeError::SecretMissingError)?;
            log::info!("Reading secret {}", secret_name);

            let secrets_api = Api::<Secret>::default_namespaced(context.client.clone());
            let personal_access_token = secrets_api
                .get(&secret_name)
                .await
                .iter()
                .flat_map(|s| s.data.clone())
                .flat_map(|m| m.get("personal_access_token").map(Clone::clone))
                .flat_map(|buf| std::str::from_utf8(&buf.0).ok().map(ToString::to_string))
                .next()
                .ok_or(DatabricksKubeError::SecretMissingError)?;

            git_credentials_api::git_credentialsupdate(
                &config,
                credential_id.expect("Must have credential ID for update"),
                WorkspaceUpdateCredentials {
                    git_username: credential.git_username,
                    git_provider: credential.git_provider,
                    personal_access_token: Some(personal_access_token),
                    ..WorkspaceUpdateCredentials::default()
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.credential = self.remote_get(context.clone()).next().await.unwrap()?;
            yield with_response;
        }
        .boxed()
    }

    fn remote_delete(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        let credential_id = self.spec().credential.credential_id;

        try_stream! {
            let config = WorkspaceCredentialInfo::get_rest_config(context.clone()).await.unwrap();
            git_credentials_api::git_credentialsdelete(
                &config,
                credential_id.expect("Must have credential ID for delete")
            )
            .await?;

            yield ()
        }
        .boxed()
    }
}
