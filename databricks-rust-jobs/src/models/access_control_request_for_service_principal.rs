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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlRequestForServicePrincipal {
    /// Name of an Azure service principal.
    #[serde(rename = "service_principal_name", skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[serde(rename = "permission_level", skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<Box<crate::models::PermissionLevel>>,
}

impl AccessControlRequestForServicePrincipal {
    pub fn new() -> AccessControlRequestForServicePrincipal {
        AccessControlRequestForServicePrincipal {
            service_principal_name: None,
            permission_level: None,
        }
    }
}

