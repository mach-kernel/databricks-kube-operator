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
pub struct SqlChannel {
    /// 
    #[serde(rename = "dbsql_version", skip_serializing_if = "Option::is_none")]
    pub dbsql_version: Option<String>,
    /// 
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}

impl SqlChannel {
    pub fn new() -> SqlChannel {
        SqlChannel {
            dbsql_version: None,
            name: None,
        }
    }
}

/// 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "CHANNEL_NAME_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "CHANNEL_NAME_PREVIEW")]
    Preview,
    #[serde(rename = "CHANNEL_NAME_CURRENT")]
    Current,
    #[serde(rename = "CHANNEL_NAME_PREVIOUS")]
    Previous,
    #[serde(rename = "CHANNEL_NAME_CUSTOM")]
    Custom,
}

impl Default for Name {
    fn default() -> Name {
        Self::Unspecified
    }
}

