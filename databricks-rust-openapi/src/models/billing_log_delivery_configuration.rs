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
pub struct BillingLogDeliveryConfiguration {
    /// Databricks log delivery configuration ID.
    #[serde(rename = "config_id", skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// The ID for a method:storage/create  that represents the S3 bucket with bucket policy as described in the main billable usage documentation page. See [Configure billable usage delivery](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html).
    #[serde(rename = "storage_configuration_id", skip_serializing_if = "Option::is_none")]
    pub storage_configuration_id: Option<String>,
    #[serde(rename = "output_format", skip_serializing_if = "Option::is_none")]
    pub output_format: Option<crate::models::BillingOutputFormat>,
    /// This field applies only if `log_type` is `BILLABLE_USAGE`. This is the optional start month and year for delivery, specified in `YYYY-MM` format. Defaults to current year and month.  `BILLABLE_USAGE` logs are not available for usage before March 2019 (`2019-03`).
    #[serde(rename = "delivery_start_time", skip_serializing_if = "Option::is_none")]
    pub delivery_start_time: Option<String>,
    /// Time in epoch milliseconds when the log delivery configuration was created.
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "workspace_ids_filter", skip_serializing_if = "Option::is_none")]
    pub workspace_ids_filter: Option<Vec<i64>>,
    /// The optional human-readable name of the log delivery configuration. Defaults to empty.
    #[serde(rename = "config_name", skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
    /// Time in epoch milliseconds when the log delivery configuration was updated.
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::BillingLogDeliveryConfigStatus>,
    /// The ID for a method:credentials/create that represents the AWS IAM role with policy and trust relationship as described in the main billable usage documentation page. See [Configure billable usage delivery](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html).
    #[serde(rename = "credentials_id", skip_serializing_if = "Option::is_none")]
    pub credentials_id: Option<String>,
    #[serde(rename = "log_delivery_status", skip_serializing_if = "Option::is_none")]
    pub log_delivery_status: Option<Box<crate::models::BillingLogDeliveryStatus>>,
    /// The optional delivery path prefix within Amazon S3 storage. Defaults to empty, which means that logs are delivered to the root of the bucket. This must be a valid S3 object key. This must not start or end with a slash character.
    #[serde(rename = "delivery_path_prefix", skip_serializing_if = "Option::is_none")]
    pub delivery_path_prefix: Option<String>,
    /// The Databricks account ID that hosts the log delivery configuration.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "log_type", skip_serializing_if = "Option::is_none")]
    pub log_type: Option<crate::models::BillingLogType>,
}

impl BillingLogDeliveryConfiguration {
    pub fn new() -> BillingLogDeliveryConfiguration {
        BillingLogDeliveryConfiguration {
            config_id: None,
            storage_configuration_id: None,
            output_format: None,
            delivery_start_time: None,
            creation_time: None,
            workspace_ids_filter: None,
            config_name: None,
            update_time: None,
            status: None,
            credentials_id: None,
            log_delivery_status: None,
            delivery_path_prefix: None,
            account_id: None,
            log_type: None,
        }
    }
}


