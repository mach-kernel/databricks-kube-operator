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
pub struct SharingRecipientProfile {
    /// The token used to authorize the recipient.
    #[serde(rename = "bearer_token", skip_serializing_if = "Option::is_none")]
    pub bearer_token: Option<String>,
    /// The endpoint for the share to be used by the recipient.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The version number of the recipient's credentials on a share.
    #[serde(rename = "share_credentials_version", skip_serializing_if = "Option::is_none")]
    pub share_credentials_version: Option<i32>,
}

impl SharingRecipientProfile {
    pub fn new() -> SharingRecipientProfile {
        SharingRecipientProfile {
            bearer_token: None,
            endpoint: None,
            share_credentials_version: None,
        }
    }
}


