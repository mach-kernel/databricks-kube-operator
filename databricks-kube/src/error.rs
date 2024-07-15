use serde::Serialize;
use serde_json::to_value;
use std::fmt::{Debug, Display};
use thiserror::Error;

use databricks_rust_git_credentials::apis::{
    Error as GitCredentialAPIError, ResponseContent as GitCredentialsResponseContent,
};
use databricks_rust_jobs::apis::{Error as JobsAPIError, ResponseContent as JobsResponseContent};
use databricks_rust_repos::apis::{
    Error as ReposAPIError, ResponseContent as ReposResponseContent,
};
use databricks_rust_secrets::apis::{
    Error as SecretsAPIError, ResponseContent as SecretsResponseContent,
};
use kube::runtime::finalizer::Error as KubeFinalizerError;

#[macro_export]
macro_rules! openapi_error_glue {
    ($error:ident, $response:ident) => {
        impl<T> From<$error<T>> for DatabricksKubeError
        where
            T: Debug + Serialize + 'static,
        {
            fn from(e: $error<T>) -> Self {
                match e {
                    $error::ResponseError($response {
                        status,
                        content,
                        entity,
                    }) => Self::APIError(OpenAPIError::ResponseError(SerializableResponseContent {
                        status,
                        content,
                        entity: entity.and_then(|e| to_value(e).ok()),
                    })),
                    $error::Io(e) => Self::APIError(OpenAPIError::Io(e)),
                    $error::Serde(e) => Self::APIError(OpenAPIError::Serde(e)),
                    $error::Reqwest(e) => Self::APIError(OpenAPIError::Reqwest(e)),
                }
            }
        }
    };
}

openapi_error_glue!(JobsAPIError, JobsResponseContent);
openapi_error_glue!(GitCredentialAPIError, GitCredentialsResponseContent);
openapi_error_glue!(ReposAPIError, ReposResponseContent);
openapi_error_glue!(SecretsAPIError, SecretsResponseContent);

impl<T> From<KubeFinalizerError<T>> for DatabricksKubeError
where
    T: Debug,
    T: std::error::Error,
{
    fn from(e: KubeFinalizerError<T>) -> Self {
        Self::FinalizerError(format!("{:?}", e))
    }
}

#[derive(Debug)]
pub struct SerializableResponseContent {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<serde_json::Value>,
}

#[derive(Error, Debug)]
pub enum OpenAPIError {
    Reqwest(#[from] reqwest::Error),
    Serde(#[from] serde_json::Error),
    Io(#[from] std::io::Error),
    ResponseError(SerializableResponseContent),
}

impl Display for OpenAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAPIError::Reqwest(err) => Display::fmt(err, f),
            OpenAPIError::Serde(err) => Display::fmt(err, f),
            OpenAPIError::Io(err) => Display::fmt(err, f),
            OpenAPIError::ResponseError(SerializableResponseContent {
                status,
                content,
                entity,
            }) => write!(f, "API response error: {} {} {:?}", status, content, entity),
        }
    }
}

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum DatabricksKubeError {
    #[error("Error calling Databricks API: {0}")]
    APIError(#[from] OpenAPIError),

    #[error("Timed out waiting for config map {0}")]
    ConfigMapMissingError(String),

    #[error("Controller reconciliation failed")]
    ControllerError(String),

    #[error("Timed out waiting for credentials")]
    CredentialsError,

    #[error(
        "Timed out while waiting for CRD: {0}\n\nGet all CRDs by running:\ncargo run --bin crd_gen"
    )]
    CRDMissingError(String),

    #[error("Finalizer error: {0}")]
    FinalizerError(String),

    #[error("The resource ID is unset")]
    IDUnsetError,

    #[error("Unable to update resource for: {0}")]
    ResourceUpdateError(String),

    #[error("Unable to get resource status for: {0}")]
    ResourceStatusError(String),

    #[error("Secret {0} is missing")]
    SecretMissingError(String),

    #[error("Shutdown requested: {0}")]
    Shutdown(String),
}
