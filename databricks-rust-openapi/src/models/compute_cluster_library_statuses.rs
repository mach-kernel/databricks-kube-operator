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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeClusterLibraryStatuses {
    /// Unique identifier for the cluster.
    #[serde(rename = "cluster_id", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "library_statuses", skip_serializing_if = "Option::is_none")]
    pub library_statuses: Option<Vec<crate::models::ComputeLibraryFullStatus>>,
}

impl ComputeClusterLibraryStatuses {
    pub fn new() -> ComputeClusterLibraryStatuses {
        ComputeClusterLibraryStatuses {
            cluster_id: None,
            library_statuses: None,
        }
    }
}


