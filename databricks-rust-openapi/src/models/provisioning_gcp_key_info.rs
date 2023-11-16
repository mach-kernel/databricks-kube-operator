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
pub struct ProvisioningGcpKeyInfo {
    /// The GCP KMS key's resource name
    #[serde(rename = "kms_key_id")]
    pub kms_key_id: String,
}

impl ProvisioningGcpKeyInfo {
    pub fn new(kms_key_id: String) -> ProvisioningGcpKeyInfo {
        ProvisioningGcpKeyInfo {
            kms_key_id,
        }
    }
}


