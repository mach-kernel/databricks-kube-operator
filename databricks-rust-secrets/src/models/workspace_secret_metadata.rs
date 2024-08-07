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

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSecretMetadata {
    /// A unique name to identify the secret.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The last updated timestamp (in milliseconds) for the secret.
    #[serde(rename = "last_updated_timestamp", skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<i64>,
}

impl WorkspaceSecretMetadata {
    pub fn new() -> WorkspaceSecretMetadata {
        WorkspaceSecretMetadata {
            key: None,
            last_updated_timestamp: None,
        }
    }
}

