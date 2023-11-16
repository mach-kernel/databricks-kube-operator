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

/// ProvisioningWorkspaceStatus : The status of the workspace. For workspace creation, usually it is set to `PROVISIONING` initially. Continue to check the status until the status is `RUNNING`. 

/// The status of the workspace. For workspace creation, usually it is set to `PROVISIONING` initially. Continue to check the status until the status is `RUNNING`. 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProvisioningWorkspaceStatus {
    #[serde(rename = "NOT_PROVISIONED")]
    NotProvisioned,
    #[serde(rename = "PROVISIONING")]
    Provisioning,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "BANNED")]
    Banned,
    #[serde(rename = "CANCELLING")]
    Cancelling,

}

impl ToString for ProvisioningWorkspaceStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NotProvisioned => String::from("NOT_PROVISIONED"),
            Self::Provisioning => String::from("PROVISIONING"),
            Self::Running => String::from("RUNNING"),
            Self::Failed => String::from("FAILED"),
            Self::Banned => String::from("BANNED"),
            Self::Cancelling => String::from("CANCELLING"),
        }
    }
}

impl Default for ProvisioningWorkspaceStatus {
    fn default() -> ProvisioningWorkspaceStatus {
        Self::NotProvisioned
    }
}




