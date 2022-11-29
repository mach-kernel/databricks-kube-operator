use std::sync::Arc;
use std::{pin::Pin, time::SystemTime};

use crate::context::Context;
use crate::traits::rest_config::RestConfig;
use crate::{error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource};

use databricks_rust_git_credentials::{
    apis::default_api,
    models::{
        CreateCredentialRequest, GetCredentialResponse as APICredential, GetCredentialsResponse,
        UpdateCredentialRequest,
    },
};

use async_stream::try_stream;
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
    pub credential: APICredential,
    // The user provides an API token during a create request, but it
    // is otherwise no longer retrievable. Even with Helm/GitOps workflow,
    // the secret doesn't have to be checked in and could come from AWS
    // SSM or secrets manager.
    pub secret_name: Option<String>,
}

/// API -> CRD
impl From<APICredential> for GitCredential {
    fn from(credential: APICredential) -> Self {
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
impl From<GitCredential> for APICredential {
    fn from(value: GitCredential) -> Self {
        value.spec().credential.clone()
    }
}

impl RemoteAPIResource<APICredential> for GitCredential {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<APICredential, DatabricksKubeError>> + Send>> {
        try_stream! {
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();

            while let GetCredentialsResponse {
                credentials,
                ..
            } = default_api::get_git_credential_list(&config).await? {
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
    ) -> Pin<Box<dyn Stream<Item = Result<APICredential, DatabricksKubeError>> + Send>> {
        let credential_id =
            self.spec()
                .credential
                .credential_id
                .ok_or(DatabricksKubeError::APIError(
                    "Remote resource cannot exist".to_string(),
                ));

        try_stream! {
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();

            let res = default_api::get_git_credential(&config, &credential_id?.to_string()).await?;

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
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();

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

            let new_credential = default_api::create_git_credential(
                &config,
                CreateCredentialRequest {
                    personal_access_token,
                    git_username: credential.git_username.unwrap(),
                    git_provider: credential.git_provider.unwrap(),
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.credential = new_credential;
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
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();

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

            default_api::update_git_credential(
                &config,
                &credential_id.unwrap().to_string(),
                UpdateCredentialRequest {
                    git_username: credential.git_username,
                    git_provider: credential.git_provider,
                    personal_access_token,
                    ..UpdateCredentialRequest::default()
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
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();
            default_api::delete_git_credential(
                &config,
                &credential_id.map(|i| i.to_string()).unwrap()
            )
            .await?;

            yield ()
        }
        .boxed()
    }
}
