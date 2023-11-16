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

/// SqlResultSchema : Schema is an ordered list of column descriptions.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlResultSchema {
    #[serde(rename = "column_count", skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<crate::models::SqlColumnInfo>>,
}

impl SqlResultSchema {
    /// Schema is an ordered list of column descriptions.
    pub fn new() -> SqlResultSchema {
        SqlResultSchema {
            column_count: None,
            columns: None,
        }
    }
}


