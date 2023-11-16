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
pub struct IamServicePrincipal {
    /// If this user is active
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// UUID relating to the service principal
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// String that represents a concatenation of given and family names.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "entitlements", skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<crate::models::IamComplexValue>>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "groups", default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<serde_json::Value>,
    /// Databricks service principal ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "roles", default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<serde_json::Value>,
}

impl IamServicePrincipal {
    pub fn new() -> IamServicePrincipal {
        IamServicePrincipal {
            active: None,
            application_id: None,
            display_name: None,
            entitlements: None,
            external_id: None,
            groups: None,
            id: None,
            roles: None,
        }
    }
}


