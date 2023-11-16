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

/// ProvisioningGkeConfig : The configurations for the GKE cluster of a Databricks workspace.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisioningGkeConfig {
    /// Specifies the network connectivity types for the GKE nodes and the GKE master network.  Set to `PRIVATE_NODE_PUBLIC_MASTER` for a private GKE cluster for the workspace. The GKE nodes will not have public IPs.  Set to `PUBLIC_NODE_PUBLIC_MASTER` for a public GKE cluster. The nodes of a public GKE cluster have public IP addresses. 
    #[serde(rename = "connectivity_type", skip_serializing_if = "Option::is_none")]
    pub connectivity_type: Option<ConnectivityType>,
    /// The IP range from which to allocate GKE cluster master resources. This field will be ignored if GKE private cluster is not enabled.  It must be exactly as big as `/28`.
    #[serde(rename = "master_ip_range", skip_serializing_if = "Option::is_none")]
    pub master_ip_range: Option<String>,
}

impl ProvisioningGkeConfig {
    /// The configurations for the GKE cluster of a Databricks workspace.
    pub fn new() -> ProvisioningGkeConfig {
        ProvisioningGkeConfig {
            connectivity_type: None,
            master_ip_range: None,
        }
    }
}

/// Specifies the network connectivity types for the GKE nodes and the GKE master network.  Set to `PRIVATE_NODE_PUBLIC_MASTER` for a private GKE cluster for the workspace. The GKE nodes will not have public IPs.  Set to `PUBLIC_NODE_PUBLIC_MASTER` for a public GKE cluster. The nodes of a public GKE cluster have public IP addresses. 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectivityType {
    #[serde(rename = "PRIVATE_NODE_PUBLIC_MASTER")]
    PrivateNodePublicMaster,
    #[serde(rename = "PUBLIC_NODE_PUBLIC_MASTER")]
    PublicNodePublicMaster,
}

impl Default for ConnectivityType {
    fn default() -> ConnectivityType {
        Self::PrivateNodePublicMaster
    }
}

