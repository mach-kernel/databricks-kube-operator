use std::fmt::Display;
use std::error::Error;

use crate::config::CONFIGMAP_NAME;

#[derive(Debug)]
pub enum DatabricksKubeError {
    ConfigMapMissingError,
}

impl Display for DatabricksKubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DatabricksKubeError::ConfigMapMissingError => format!(
                "Timed out while waiting for config map: {}",
                *CONFIGMAP_NAME
            )
        };
        write!(f, "{}", msg)
    }
}

impl Error for DatabricksKubeError {}