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

/// CatalogProvisioningState : Status of an asynchronously provisioned resource.

/// Status of an asynchronously provisioned resource.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CatalogProvisioningState {
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "PROVISIONING")]
    Provisioning,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "DELETING")]
    Deleting,

}

impl ToString for CatalogProvisioningState {
    fn to_string(&self) -> String {
        match self {
            Self::StateUnspecified => String::from("STATE_UNSPECIFIED"),
            Self::Provisioning => String::from("PROVISIONING"),
            Self::Active => String::from("ACTIVE"),
            Self::Failed => String::from("FAILED"),
            Self::Deleting => String::from("DELETING"),
        }
    }
}

impl Default for CatalogProvisioningState {
    fn default() -> CatalogProvisioningState {
        Self::StateUnspecified
    }
}




