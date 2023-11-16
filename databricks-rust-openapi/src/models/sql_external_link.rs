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
pub struct SqlExternalLink {
    /// Number of bytes in the result chunk.
    #[serde(rename = "byte_count", skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
    /// Position within the sequence of result set chunks.
    #[serde(rename = "chunk_index", skip_serializing_if = "Option::is_none")]
    pub chunk_index: Option<i32>,
    /// Indicates date-time that the given external link will expire and become invalid, after which point a new `external_link` must be requested. 
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Pre-signed URL pointing to a chunk of result data, hosted by an external service, with a short expiration time (< 1 hour). 
    #[serde(rename = "external_link", skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,
    /// When fetching, gives `chunk_index` for the _next_ chunk; if absent, indicates there are no more chunks.
    #[serde(rename = "next_chunk_index", skip_serializing_if = "Option::is_none")]
    pub next_chunk_index: Option<i32>,
    /// When fetching, gives `internal_link` for the _next_ chunk; if absent, indicates there are no more chunks.
    #[serde(rename = "next_chunk_internal_link", skip_serializing_if = "Option::is_none")]
    pub next_chunk_internal_link: Option<String>,
    /// Number of rows within the result chunk.
    #[serde(rename = "row_count", skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i64>,
    /// Starting row offset within the result set.
    #[serde(rename = "row_offset", skip_serializing_if = "Option::is_none")]
    pub row_offset: Option<i64>,
}

impl SqlExternalLink {
    pub fn new() -> SqlExternalLink {
        SqlExternalLink {
            byte_count: None,
            chunk_index: None,
            expiration: None,
            external_link: None,
            next_chunk_index: None,
            next_chunk_internal_link: None,
            row_count: None,
            row_offset: None,
        }
    }
}


