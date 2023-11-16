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
pub struct IamAccessControlRequest {
    /// name of the group
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "permission_level", skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<crate::models::IamPermissionLevel>,
    /// name of the service principal
    #[serde(rename = "service_principal_name", skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    /// name of the user
    #[serde(rename = "user_name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl IamAccessControlRequest {
    pub fn new() -> IamAccessControlRequest {
        IamAccessControlRequest {
            group_name: None,
            permission_level: None,
            service_principal_name: None,
            user_name: None,
        }
    }
}


