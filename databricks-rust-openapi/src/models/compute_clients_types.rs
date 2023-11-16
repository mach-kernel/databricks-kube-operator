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
pub struct ComputeClientsTypes {
    /// With jobs set, the cluster can be used for jobs
    #[serde(rename = "jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<bool>,
    /// With notebooks set, this cluster can be used for notebooks
    #[serde(rename = "notebooks", skip_serializing_if = "Option::is_none")]
    pub notebooks: Option<bool>,
}

impl ComputeClientsTypes {
    pub fn new() -> ComputeClientsTypes {
        ComputeClientsTypes {
            jobs: None,
            notebooks: None,
        }
    }
}


