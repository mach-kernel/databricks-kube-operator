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

/// WorkspaceRepoPermissionLevel : Permission level

/// Permission level
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkspaceRepoPermissionLevel {
    #[serde(rename = "CAN_MANAGE")]
    Manage,
    #[serde(rename = "CAN_EDIT")]
    Edit,
    #[serde(rename = "CAN_RUN")]
    Run,
    #[serde(rename = "CAN_READ")]
    Read,

}

impl ToString for WorkspaceRepoPermissionLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Manage => String::from("CAN_MANAGE"),
            Self::Edit => String::from("CAN_EDIT"),
            Self::Run => String::from("CAN_RUN"),
            Self::Read => String::from("CAN_READ"),
        }
    }
}

impl Default for WorkspaceRepoPermissionLevel {
    fn default() -> WorkspaceRepoPermissionLevel {
        Self::Manage
    }
}




