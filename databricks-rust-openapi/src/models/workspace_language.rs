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

/// WorkspaceLanguage : The language of the object. This value is set only if the object type is `NOTEBOOK`.

/// The language of the object. This value is set only if the object type is `NOTEBOOK`.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkspaceLanguage {
    #[serde(rename = "SCALA")]
    Scala,
    #[serde(rename = "PYTHON")]
    Python,
    #[serde(rename = "SQL")]
    Sql,
    #[serde(rename = "R")]
    R,

}

impl ToString for WorkspaceLanguage {
    fn to_string(&self) -> String {
        match self {
            Self::Scala => String::from("SCALA"),
            Self::Python => String::from("PYTHON"),
            Self::Sql => String::from("SQL"),
            Self::R => String::from("R"),
        }
    }
}

impl Default for WorkspaceLanguage {
    fn default() -> WorkspaceLanguage {
        Self::Scala
    }
}




