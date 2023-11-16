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
pub struct IamPasswordAccessControlResponse {
    #[serde(rename = "all_permissions", skip_serializing_if = "Option::is_none")]
    pub all_permissions: Option<Vec<crate::models::IamPasswordPermission>>,
    /// Display name of the user or service principal.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// name of the group
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Name of the service principal.
    #[serde(rename = "service_principal_name", skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    /// name of the user
    #[serde(rename = "user_name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl IamPasswordAccessControlResponse {
    pub fn new() -> IamPasswordAccessControlResponse {
        IamPasswordAccessControlResponse {
            all_permissions: None,
            display_name: None,
            group_name: None,
            service_principal_name: None,
            user_name: None,
        }
    }
}


