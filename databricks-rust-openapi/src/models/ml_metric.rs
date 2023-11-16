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
pub struct MlMetric {
    /// Key identifying this metric.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Step at which to log the metric.
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
    /// The timestamp at which this metric was recorded.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Value associated with this metric.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl MlMetric {
    pub fn new() -> MlMetric {
        MlMetric {
            key: None,
            step: None,
            timestamp: None,
            value: None,
        }
    }
}


