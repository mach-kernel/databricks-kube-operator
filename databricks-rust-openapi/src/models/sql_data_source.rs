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

/// SqlDataSource : A JSON object representing a DBSQL data source / SQL warehouse.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDataSource {
    /// The ID of the associated SQL warehouse, if this data source is backed by a SQL warehouse.
    #[serde(rename = "warehouse_id", skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    /// Data source ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The string name of this data source / SQL warehouse as it appears in the Databricks SQL web application.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Reserved for internal use.
    #[serde(rename = "pause_reason", skip_serializing_if = "Option::is_none")]
    pub pause_reason: Option<String>,
    /// Reserved for internal use.
    #[serde(rename = "paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<i32>,
    /// Reserved for internal use.
    #[serde(rename = "supports_auto_limit", skip_serializing_if = "Option::is_none")]
    pub supports_auto_limit: Option<bool>,
    /// The type of data source. For SQL warehouses, this will be `databricks_internal`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Reserved for internal use.
    #[serde(rename = "syntax", skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// Reserved for internal use.
    #[serde(rename = "view_only", skip_serializing_if = "Option::is_none")]
    pub view_only: Option<bool>,
}

impl SqlDataSource {
    /// A JSON object representing a DBSQL data source / SQL warehouse.
    pub fn new() -> SqlDataSource {
        SqlDataSource {
            warehouse_id: None,
            id: None,
            name: None,
            pause_reason: None,
            paused: None,
            supports_auto_limit: None,
            r#type: None,
            syntax: None,
            view_only: None,
        }
    }
}


