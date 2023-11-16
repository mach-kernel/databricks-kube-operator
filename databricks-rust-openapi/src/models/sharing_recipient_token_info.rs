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
pub struct SharingRecipientTokenInfo {
    /// Full activation URL to retrieve the access token. It will be empty if the token is already retrieved.
    #[serde(rename = "activation_url", skip_serializing_if = "Option::is_none")]
    pub activation_url: Option<String>,
    /// Time at which this recipient Token was created, in epoch milliseconds.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Username of recipient token creator.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Expiration timestamp of the token in epoch milliseconds.
    #[serde(rename = "expiration_time", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    /// Unique ID of the recipient token.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Time at which this recipient Token was updated, in epoch milliseconds.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Username of recipient Token updater.
    #[serde(rename = "updated_by", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

impl SharingRecipientTokenInfo {
    pub fn new() -> SharingRecipientTokenInfo {
        SharingRecipientTokenInfo {
            activation_url: None,
            created_at: None,
            created_by: None,
            expiration_time: None,
            id: None,
            updated_at: None,
            updated_by: None,
        }
    }
}


