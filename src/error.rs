use std::error::Error;
use std::fmt::Display;

use crate::config::CONFIGMAP_NAME;

#[derive(Debug)]
pub enum DatabricksKubeError {
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
            )
        };
        write!(f, "{}", msg)
    }
}

impl Error for DatabricksKubeError {}
