use std::error::Error;
use std::fmt::{Debug, Display};

use crate::config::CONFIGMAP_NAME;

use databricks_rust_jobs::apis::Error as JobsAPIError;
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
    CRDMissingError(String),
}

impl Display for DatabricksKubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DatabricksKubeError::ConfigMapMissingError => format!(
                "Timed out while waiting for config map: {}",
                *CONFIGMAP_NAME
            ),
            DatabricksKubeError::CRDMissingError(crd) => format!(
                "Timed out while waiting for CRD: {}\n\nGet all CRDs by running:\ncargo run --bin crd_gen",
                crd
            ),
            DatabricksKubeError::APIError(err )=> format!(
                "Error calling Databricks API:\n{}",
                err
            )
        };
        write!(f, "{}", msg)
    }
}

impl Error for DatabricksKubeError {}
