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

/// SqlAlertOptions : Alert configuration options.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlAlertOptions {
    /// Name of column in the query result to compare in alert evaluation.
    #[serde(rename = "column")]
    pub column: String,
    /// Custom body of alert notification, if it exists. See [here](https://Docsdatabricks.com/sql/user/alerts/index.html) for custom templating instructions.
    #[serde(rename = "custom_body", skip_serializing_if = "Option::is_none")]
    pub custom_body: Option<String>,
    /// Custom subject of alert notification, if it exists. This includes email subject, Slack notification header, etc. See [here](https://Docsdatabricks.com/sql/user/alerts/index.html) for custom templating instructions.
    #[serde(rename = "custom_subject", skip_serializing_if = "Option::is_none")]
    pub custom_subject: Option<String>,
    /// Whether or not the alert is muted. If an alert is muted, it will not notify users and notification destinations when triggered.
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    /// Operator used to compare in alert evaluation: `>`, `>=`, `<`, `<=`, `==`, `!=`
    #[serde(rename = "op")]
    pub op: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

impl SqlAlertOptions {
    /// Alert configuration options.
    pub fn new(column: String, op: String, value: serde_json::Value) -> SqlAlertOptions {
        SqlAlertOptions {
            column,
            custom_body: None,
            custom_subject: None,
            muted: None,
            op,
            value,
        }
    }
}


