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

/// BillingLogDeliveryStatus : Databricks log delivery status.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingLogDeliveryStatus {
    /// The UTC time for the latest log delivery attempt.
    #[serde(rename = "last_attempt_time", skip_serializing_if = "Option::is_none")]
    pub last_attempt_time: Option<String>,
    /// The UTC time for the latest successful log delivery.
    #[serde(rename = "last_successful_attempt_time", skip_serializing_if = "Option::is_none")]
    pub last_successful_attempt_time: Option<String>,
    /// Informative message about the latest log delivery attempt. If the log delivery fails with USER_FAILURE, error details will be provided for fixing misconfigurations in cloud permissions.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::BillingDeliveryStatus>,
}

impl BillingLogDeliveryStatus {
    /// Databricks log delivery status.
    pub fn new() -> BillingLogDeliveryStatus {
        BillingLogDeliveryStatus {
            last_attempt_time: None,
            last_successful_attempt_time: None,
            message: None,
            status: None,
        }
    }
}


