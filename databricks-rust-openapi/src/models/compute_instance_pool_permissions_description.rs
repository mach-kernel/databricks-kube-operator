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
pub struct ComputeInstancePoolPermissionsDescription {
    /// 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "permission_level", skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<crate::models::ComputeInstancePoolPermissionLevel>,
}

impl ComputeInstancePoolPermissionsDescription {
    pub fn new() -> ComputeInstancePoolPermissionsDescription {
        ComputeInstancePoolPermissionsDescription {
            description: None,
            permission_level: None,
        }
    }
}


