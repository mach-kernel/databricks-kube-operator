use std::error::Error;
use std::fmt::{Debug, Display};

use crate::context::CONFIGMAP_NAME;

use databricks_rust_jobs::apis::Error as JobsAPIError;
use tokio_graceful_shutdown::errors::GracefulShutdownError;
impl<T> From<JobsAPIError<T>> for DatabricksKubeError
where
    T: Debug,
{
    fn from(e: JobsAPIError<T>) -> Self {
        Self::APIError(format!("{}", e))
    }
}

#[derive(Debug)]
pub enum DatabricksKubeError {
    APIError(String),
    ConfigMapMissingError,
    CredentialsError,
    CRDMissingError(String),
    Shutdown(String),
}

impl Display for DatabricksKubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DatabricksKubeError::APIError(err )=> format!(
                "Error calling Databricks API:\n{}",
                err
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
            )
        };
        write!(f, "{}", msg)
    }
}

impl Error for DatabricksKubeError {}
