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

/// ProvisioningCustomerFacingGcpCloudResourceContainer : The general workspace configurations that are specific to Google Cloud.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisioningCustomerFacingGcpCloudResourceContainer {
    /// The Google Cloud project ID, which the workspace uses to instantiate cloud resources for your workspace.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ProvisioningCustomerFacingGcpCloudResourceContainer {
    /// The general workspace configurations that are specific to Google Cloud.
    pub fn new() -> ProvisioningCustomerFacingGcpCloudResourceContainer {
        ProvisioningCustomerFacingGcpCloudResourceContainer {
            project_id: None,
        }
    }
}


