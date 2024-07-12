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
pub struct WorkspaceListAclsResponse {
    /// The associated ACLs rule applied to principals in the given scope.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::WorkspaceAclItem>>,
}

impl WorkspaceListAclsResponse {
    pub fn new() -> WorkspaceListAclsResponse {
        WorkspaceListAclsResponse {
            items: None,
        }
    }
}

