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

/// SqlWarehousePermissionLevel : Permission level

/// Permission level
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SqlWarehousePermissionLevel {
    #[serde(rename = "CAN_MANAGE")]
    CanManage,
    #[serde(rename = "IS_OWNER")]
    IsOwner,
    #[serde(rename = "CAN_USE")]
    CanUse,

}

impl ToString for SqlWarehousePermissionLevel {
    fn to_string(&self) -> String {
        match self {
            Self::CanManage => String::from("CAN_MANAGE"),
            Self::IsOwner => String::from("IS_OWNER"),
            Self::CanUse => String::from("CAN_USE"),
        }
    }
}

impl Default for SqlWarehousePermissionLevel {
    fn default() -> SqlWarehousePermissionLevel {
        Self::CanManage
    }
}




