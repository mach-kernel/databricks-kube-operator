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
pub struct SqlAlertQuery {
    /// The ID of the user who created this query.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// Indicates whether the query is trashed. Trashed queries can't be used in dashboards, or appear in search results. If this boolean is `true`, the `options` property for this query includes a `moved_to_trash_at` timestamp. Trashed queries are permanently deleted after 30 days.
    #[serde(rename = "is_archived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// The text of the query to be run.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Query ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The title of this query that appears in list views, widget headings, and on the query page.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The timestamp at which this query was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Text parameter types are not safe from SQL injection for all types of data source. Set this Boolean parameter to `true` if a query either does not use any text type parameters or uses a data source type where text type parameters are handled safely.
    #[serde(rename = "is_safe", skip_serializing_if = "Option::is_none")]
    pub is_safe: Option<bool>,
    /// Whether the query is a draft. Draft queries only appear in list views for their owners. Visualizations from draft queries cannot appear on dashboards.
    #[serde(rename = "is_draft", skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// Data source ID.
    #[serde(rename = "data_source_id", skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// The timestamp when this query was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::SqlQueryOptions>>,
    /// General description that conveys additional information about this query such as usage notes.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SqlAlertQuery {
    pub fn new() -> SqlAlertQuery {
        SqlAlertQuery {
            user_id: None,
            is_archived: None,
            query: None,
            tags: None,
            id: None,
            name: None,
            updated_at: None,
            is_safe: None,
            is_draft: None,
            data_source_id: None,
            created_at: None,
            options: None,
            description: None,
        }
    }
}


