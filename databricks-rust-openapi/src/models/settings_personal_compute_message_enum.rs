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

/// SettingsPersonalComputeMessageEnum : ON: Grants all users in all workspaces access to the Personal Compute default policy, allowing all users to create single-machine compute resources. DELEGATE: Moves access control for the Personal Compute default policy to individual workspaces and requires a workspace’s users or groups to be added to the ACLs of that workspace’s Personal Compute default policy before they will be able to create compute resources through that policy. 

/// ON: Grants all users in all workspaces access to the Personal Compute default policy, allowing all users to create single-machine compute resources. DELEGATE: Moves access control for the Personal Compute default policy to individual workspaces and requires a workspace’s users or groups to be added to the ACLs of that workspace’s Personal Compute default policy before they will be able to create compute resources through that policy. 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SettingsPersonalComputeMessageEnum {
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "DELEGATE")]
    Delegate,

}

impl ToString for SettingsPersonalComputeMessageEnum {
    fn to_string(&self) -> String {
        match self {
            Self::On => String::from("ON"),
            Self::Delegate => String::from("DELEGATE"),
        }
    }
}

impl Default for SettingsPersonalComputeMessageEnum {
    fn default() -> SettingsPersonalComputeMessageEnum {
        Self::On
    }
}




