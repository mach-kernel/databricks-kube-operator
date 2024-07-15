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
pub struct WorkspacePutAcl {
    /// The permission level applied to the principal.
    #[serde(rename = "permission")]
    pub permission: models::WorkspaceAclPermission,
    /// The principal in which the permission is applied.
    #[serde(rename = "principal")]
    pub principal: String,
    /// The name of the scope to apply permissions to.
    #[serde(rename = "scope")]
    pub scope: String,
}

impl WorkspacePutAcl {
    pub fn new(permission: models::WorkspaceAclPermission, principal: String, scope: String) -> WorkspacePutAcl {
        WorkspacePutAcl {
            permission,
            principal,
            scope,
        }
    }
}

