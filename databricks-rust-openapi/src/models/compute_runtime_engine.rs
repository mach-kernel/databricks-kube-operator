use schemars::JsonSchema;
/*
 * Databricks Accounts and Workspace REST API on ALL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ComputeRuntimeEngine : Decides which runtime engine to be use, Eg. Standard vs. Photon. If unspecified, the runtime engine is inferred from spark_version.

/// Decides which runtime engine to be use, Eg. Standard vs. Photon. If unspecified, the runtime engine is inferred from spark_version.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComputeRuntimeEngine {
    #[serde(rename = "NULL")]
    Null,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "PHOTON")]
    Photon,

}

impl ToString for ComputeRuntimeEngine {
    fn to_string(&self) -> String {
        match self {
            Self::Null => String::from("NULL"),
            Self::Standard => String::from("STANDARD"),
            Self::Photon => String::from("PHOTON"),
        }
    }
}

impl Default for ComputeRuntimeEngine {
    fn default() -> ComputeRuntimeEngine {
        Self::Null
    }
}




