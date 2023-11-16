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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisioningCreateNetworkRequest {
    #[serde(rename = "gcp_network_info", skip_serializing_if = "Option::is_none")]
    pub gcp_network_info: Option<Box<crate::models::ProvisioningGcpNetworkInfo>>,
    /// The human-readable name of the network configuration.
    #[serde(rename = "network_name")]
    pub network_name: String,
    #[serde(rename = "security_group_ids", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnet_ids", skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpc_endpoints", skip_serializing_if = "Option::is_none")]
    pub vpc_endpoints: Option<Box<crate::models::ProvisioningNetworkVpcEndpoints>>,
    /// The ID of the VPC associated with this network. VPC IDs can be used in multiple network configurations.
    #[serde(rename = "vpc_id", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl ProvisioningCreateNetworkRequest {
    pub fn new(network_name: String) -> ProvisioningCreateNetworkRequest {
        ProvisioningCreateNetworkRequest {
            gcp_network_info: None,
            network_name,
            security_group_ids: None,
            subnet_ids: None,
            vpc_endpoints: None,
            vpc_id: None,
        }
    }
}


