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
pub struct ComputeDeleteCluster {
    /// The cluster to be terminated.
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
}

impl ComputeDeleteCluster {
    pub fn new(cluster_id: String) -> ComputeDeleteCluster {
        ComputeDeleteCluster {
            cluster_id,
        }
    }
}


