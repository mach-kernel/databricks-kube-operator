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




#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SparkJarTask {
    /// The full name of the class containing the main method to be executed. This class must be contained in a JAR provided as a library.  The code must use `SparkContext.getOrCreate` to obtain a Spark context; otherwise, runs of the job fail.
    #[serde(rename = "main_class_name", skip_serializing_if = "Option::is_none")]
    pub main_class_name: Option<String>,
    /// Parameters passed to the main method.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    /// Deprecated since 04/2016\\. Provide a `jar` through the `libraries` field instead. For an example, see [Create](https://docs.databricks.com/dev-tools/api/latest/jobs.html#operation/JobsCreate).
    #[serde(rename = "jar_uri", skip_serializing_if = "Option::is_none")]
    pub jar_uri: Option<String>,
}

impl SparkJarTask {
    pub fn new() -> SparkJarTask {
        SparkJarTask {
            main_class_name: None,
            parameters: None,
            jar_uri: None,
        }
    }
}


