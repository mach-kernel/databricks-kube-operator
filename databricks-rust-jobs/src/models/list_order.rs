use schemars::JsonSchema;
/*
 * Jobs API 2.1
 *
 * The Jobs API allows you to create, edit, and delete jobs. You should never hard code secrets or store them in plain text. Use the [Secrets API](https://docs.databricks.com/dev-tools/api/latest/secrets.html) to manage secrets in the [Databricks CLI](https://docs.databricks.com/dev-tools/cli/index.html). Use the [Secrets utility](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-secrets) to reference secrets in notebooks and jobs.
 *
 * The version of the OpenAPI document: 2.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListOrder : * `DESC`: Descending order. * `ASC`: Ascending order.

/// * `DESC`: Descending order. * `ASC`: Ascending order.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListOrder {
    #[serde(rename = "DESC")]
    Desc,
    #[serde(rename = "ASC")]
    Asc,

}

impl ToString for ListOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Desc => String::from("DESC"),
            Self::Asc => String::from("ASC"),
        }
    }
}

impl Default for ListOrder {
    fn default() -> ListOrder {
        Self::Desc
    }
}




