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
pub struct SharingGetRecipientSharePermissionsResponse {
    #[serde(rename = "permissions_out", skip_serializing_if = "Option::is_none")]
    pub permissions_out: Option<Vec<crate::models::SharingShareToPrivilegeAssignment>>,
}

impl SharingGetRecipientSharePermissionsResponse {
    pub fn new() -> SharingGetRecipientSharePermissionsResponse {
        SharingGetRecipientSharePermissionsResponse {
            permissions_out: None,
        }
    }
}


