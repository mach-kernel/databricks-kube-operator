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

/// MlRegisteredModelPermissionLevel : Permission level

/// Permission level
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MlRegisteredModelPermissionLevel {
    #[serde(rename = "CAN_MANAGE")]
    Manage,
    #[serde(rename = "CAN_MANAGE_PRODUCTION_VERSIONS")]
    ManageProductionVersions,
    #[serde(rename = "CAN_MANAGE_STAGING_VERSIONS")]
    ManageStagingVersions,
    #[serde(rename = "CAN_EDIT")]
    Edit,
    #[serde(rename = "CAN_READ")]
    Read,

}

impl ToString for MlRegisteredModelPermissionLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Manage => String::from("CAN_MANAGE"),
            Self::ManageProductionVersions => String::from("CAN_MANAGE_PRODUCTION_VERSIONS"),
            Self::ManageStagingVersions => String::from("CAN_MANAGE_STAGING_VERSIONS"),
            Self::Edit => String::from("CAN_EDIT"),
            Self::Read => String::from("CAN_READ"),
        }
    }
}

impl Default for MlRegisteredModelPermissionLevel {
    fn default() -> MlRegisteredModelPermissionLevel {
        Self::Manage
    }
}




