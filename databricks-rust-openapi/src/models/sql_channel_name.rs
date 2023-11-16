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

/// SqlChannelName : Name of the channel

/// Name of the channel
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SqlChannelName {
    #[serde(rename = "CHANNEL_NAME_PREVIEW")]
    Preview,
    #[serde(rename = "CHANNEL_NAME_CURRENT")]
    Current,
    #[serde(rename = "CHANNEL_NAME_PREVIOUS")]
    Previous,
    #[serde(rename = "CHANNEL_NAME_CUSTOM")]
    Custom,
    #[serde(rename = "CHANNEL_NAME_UNSPECIFIED")]
    Unspecified,

}

impl ToString for SqlChannelName {
    fn to_string(&self) -> String {
        match self {
            Self::Preview => String::from("CHANNEL_NAME_PREVIEW"),
            Self::Current => String::from("CHANNEL_NAME_CURRENT"),
            Self::Previous => String::from("CHANNEL_NAME_PREVIOUS"),
            Self::Custom => String::from("CHANNEL_NAME_CUSTOM"),
            Self::Unspecified => String::from("CHANNEL_NAME_UNSPECIFIED"),
        }
    }
}

impl Default for SqlChannelName {
    fn default() -> SqlChannelName {
        Self::Preview
    }
}




