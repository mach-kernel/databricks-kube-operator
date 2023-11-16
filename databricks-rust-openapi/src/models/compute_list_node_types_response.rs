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
pub struct ComputeListNodeTypesResponse {
    #[serde(rename = "node_types", skip_serializing_if = "Option::is_none")]
    pub node_types: Option<Vec<crate::models::ComputeNodeType>>,
}

impl ComputeListNodeTypesResponse {
    pub fn new() -> ComputeListNodeTypesResponse {
        ComputeListNodeTypesResponse {
            node_types: None,
        }
    }
}


