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
pub struct MlHttpUrlSpecWithoutSecret {
    /// Enable/disable SSL certificate validation. Default is true. For self-signed certificates, this field must be false AND the destination server must disable certificate validation as well. For security purposes, it is encouraged to perform secret validation with the HMAC-encoded portion of the payload and acknowledge the risk associated with disabling hostname validation whereby it becomes more likely that requests can be maliciously routed to an unintended host.
    #[serde(rename = "enable_ssl_verification", skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// External HTTPS URL called on event trigger (by using a POST request).
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl MlHttpUrlSpecWithoutSecret {
    pub fn new() -> MlHttpUrlSpecWithoutSecret {
        MlHttpUrlSpecWithoutSecret {
            enable_ssl_verification: None,
            url: None,
        }
    }
}


