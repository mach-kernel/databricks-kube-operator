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
pub struct WorkspaceCreateScope {
    /// The metadata for the secret scope if the type is `AZURE_KEYVAULT`
    #[serde(rename = "backend_azure_keyvault", skip_serializing_if = "Option::is_none")]
    pub backend_azure_keyvault: Option<Box<models::WorkspaceAzureKeyVaultSecretScopeMetadata>>,
    /// The principal that is initially granted `MANAGE` permission to the created scope.
    #[serde(rename = "initial_manage_principal", skip_serializing_if = "Option::is_none")]
    pub initial_manage_principal: Option<String>,
    /// Scope name requested by the user. Scope names are unique.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The backend type the scope will be created with. If not specified, will default to `DATABRICKS`
    #[serde(rename = "scope_backend_type", skip_serializing_if = "Option::is_none")]
    pub scope_backend_type: Option<models::WorkspaceScopeBackendType>,
}

impl WorkspaceCreateScope {
    pub fn new(scope: String) -> WorkspaceCreateScope {
        WorkspaceCreateScope {
            backend_azure_keyvault: None,
            initial_manage_principal: None,
            scope,
            scope_backend_type: None,
        }
    }
}

