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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceDeleteSecret {
    /// Name of the secret to delete.
    #[serde(rename = "key")]
    pub key: String,
    /// The name of the scope that contains the secret to delete.
    #[serde(rename = "scope")]
    pub scope: String,
}

impl WorkspaceDeleteSecret {
    pub fn new(key: String, scope: String) -> WorkspaceDeleteSecret {
        WorkspaceDeleteSecret {
            key,
            scope,
        }
    }
}

