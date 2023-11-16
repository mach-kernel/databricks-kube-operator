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
pub struct ComputeGetInstancePoolPermissionLevelsResponse {
    #[serde(rename = "permission_levels", skip_serializing_if = "Option::is_none")]
    pub permission_levels: Option<Vec<crate::models::ComputeInstancePoolPermissionsDescription>>,
}

impl ComputeGetInstancePoolPermissionLevelsResponse {
    pub fn new() -> ComputeGetInstancePoolPermissionLevelsResponse {
        ComputeGetInstancePoolPermissionLevelsResponse {
            permission_levels: None,
        }
    }
}


