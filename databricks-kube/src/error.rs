use std::error::Error;
use std::fmt::{Debug, Display};

use crate::context::CONFIGMAP_NAME;

use databricks_rust_git_credentials::apis::Error as GitCredentialAPIError;
use databricks_rust_jobs::apis::Error as JobsAPIError;
use databricks_rust_repos::apis::Error as ReposAPIError;

impl<T> From<JobsAPIError<T>> for DatabricksKubeError
where
    T: Debug,
{
    fn from(e: JobsAPIError<T>) -> Self {
        Self::APIError(format!("{:?}", e))
    }
}

impl<T> From<GitCredentialAPIError<T>> for DatabricksKubeError
where
    T: Debug,
{
    fn from(e: GitCredentialAPIError<T>) -> Self {
        Self::APIError(format!("{:?}", e))
    }
}

impl<T> From<ReposAPIError<T>> for DatabricksKubeError
where
    T: Debug,
{
    fn from(e: ReposAPIError<T>) -> Self {
        Self::APIError(format!("{:?}", e))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum DatabricksKubeError {
    APIError(String),
    ConfigMapMissingError,
    ControllerError(String),
    CredentialsError,
    CRDMissingError(String),
    SecretMissingError,
    Shutdown(String),
    ResourceUpdateError(String),
}

impl Display for DatabricksKubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DatabricksKubeError::APIError(err )=> format!(
                "Error calling Databricks API:\n{}",
                err
            ),
            DatabricksKubeError::ControllerError(s) => format!(
                "Controller reconciliation failed:\n{}",
                s
            ),
            DatabricksKubeError::ConfigMapMissingError => format!(
                "Timed out while waiting for config map: {}",
                *CONFIGMAP_NAME
            ),
            DatabricksKubeError::CredentialsError => "Unable to get credentials".to_owned(),
            DatabricksKubeError::CRDMissingError(crd) => format!(
                "Timed out while waiting for CRD: {}\n\nGet all CRDs by running:\ncargo run --bin crd_gen",
                crd
            ),
            DatabricksKubeError::Shutdown(s) => format!(
                "Shutdown requested, exit: {}",
                s
            ),
            DatabricksKubeError::SecretMissingError => "The secret referenced by this resource is missing".to_owned(),
            DatabricksKubeError::ResourceUpdateError(s) => format!("Unable to update K8S Resource {}", s),
        };
        write!(f, "{}", msg)
    }
}

impl Error for DatabricksKubeError {}
