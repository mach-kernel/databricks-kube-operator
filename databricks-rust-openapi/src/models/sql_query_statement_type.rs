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

/// SqlQueryStatementType : Type of statement for this query

/// Type of statement for this query
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SqlQueryStatementType {
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "ALTER")]
    Alter,
    #[serde(rename = "ANALYZE")]
    Analyze,
    #[serde(rename = "COPY")]
    Copy,
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "DESCRIBE")]
    Describe,
    #[serde(rename = "DROP")]
    Drop,
    #[serde(rename = "EXPLAIN")]
    Explain,
    #[serde(rename = "GRANT")]
    Grant,
    #[serde(rename = "INSERT")]
    Insert,
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "OPTIMIZE")]
    Optimize,
    #[serde(rename = "REFRESH")]
    Refresh,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "REVOKE")]
    Revoke,
    #[serde(rename = "SELECT")]
    Select,
    #[serde(rename = "SET")]
    Set,
    #[serde(rename = "SHOW")]
    Show,
    #[serde(rename = "TRUNCATE")]
    Truncate,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "USE")]
    Use,

}

impl ToString for SqlQueryStatementType {
    fn to_string(&self) -> String {
        match self {
            Self::Other => String::from("OTHER"),
            Self::Alter => String::from("ALTER"),
            Self::Analyze => String::from("ANALYZE"),
            Self::Copy => String::from("COPY"),
            Self::Create => String::from("CREATE"),
            Self::Delete => String::from("DELETE"),
            Self::Describe => String::from("DESCRIBE"),
            Self::Drop => String::from("DROP"),
            Self::Explain => String::from("EXPLAIN"),
            Self::Grant => String::from("GRANT"),
            Self::Insert => String::from("INSERT"),
            Self::Merge => String::from("MERGE"),
            Self::Optimize => String::from("OPTIMIZE"),
            Self::Refresh => String::from("REFRESH"),
            Self::Replace => String::from("REPLACE"),
            Self::Revoke => String::from("REVOKE"),
            Self::Select => String::from("SELECT"),
            Self::Set => String::from("SET"),
            Self::Show => String::from("SHOW"),
            Self::Truncate => String::from("TRUNCATE"),
            Self::Update => String::from("UPDATE"),
            Self::Use => String::from("USE"),
        }
    }
}

impl Default for SqlQueryStatementType {
    fn default() -> SqlQueryStatementType {
        Self::Other
    }
}




