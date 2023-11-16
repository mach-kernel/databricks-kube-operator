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
pub struct ComputeDestroyContext {
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    #[serde(rename = "contextId")]
    pub context_id: String,
}

impl ComputeDestroyContext {
    pub fn new(cluster_id: String, context_id: String) -> ComputeDestroyContext {
        ComputeDestroyContext {
            cluster_id,
            context_id,
        }
    }
}


