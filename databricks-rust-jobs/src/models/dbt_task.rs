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
pub struct DbtTask {
    /// Optional (relative) path to the project directory, if no value is provided, the root of the git repository is used.
    #[serde(rename = "project_directory", skip_serializing_if = "Option::is_none")]
    pub project_directory: Option<String>,
    /// A list of dbt commands to execute. All commands must start with `dbt`. This parameter must not be empty. A maximum of up to 10 commands can be provided.
    #[serde(rename = "commands")]
    pub commands: Vec<String>,
    /// Optional schema to write to. This parameter is only used when a warehouse_id is also provided. If not provided, the `default` schema is used.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// ID of the SQL warehouse to connect to. If provided, we automatically generate and provide the profile and connection details to dbt. It can be overridden on a per-command basis by using the `--profiles-dir` command line argument.
    #[serde(rename = "warehouse_id", skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    /// Optional name of the catalog to use. The value is the top level in the 3-level namespace of Unity Catalog (catalog / schema / relation). The catalog value can only be specified if a warehouse_id is specified. Requires dbt-databricks >= 1.1.1.
    #[serde(rename = "catalog", skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    /// Optional (relative) path to the profiles directory. Can only be specified if no warehouse_id is specified. If no warehouse_id is specified and this folder is unset, the root directory is used.
    #[serde(rename = "profiles_directory", skip_serializing_if = "Option::is_none")]
    pub profiles_directory: Option<String>,
}

impl DbtTask {
    pub fn new(commands: Vec<String>) -> DbtTask {
        DbtTask {
            project_directory: None,
            commands,
            schema: None,
            warehouse_id: None,
            catalog: None,
            profiles_directory: None,
        }
    }
}

