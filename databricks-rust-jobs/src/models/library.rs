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
pub struct Library {
    /// If jar, URI of the JAR to be installed. DBFS and S3 URIs are supported. For example: `{ \"jar\": \"dbfs:/mnt/databricks/library.jar\" }` or `{ \"jar\": \"s3://my-bucket/library.jar\" }`. If S3 is used, make sure the cluster has read access on the library. You may need to launch the cluster with an instance profile to access the S3 URI.
    #[serde(rename = "jar", skip_serializing_if = "Option::is_none")]
    pub jar: Option<String>,
    /// If egg, URI of the egg to be installed. DBFS and S3 URIs are supported. For example: `{ \"egg\": \"dbfs:/my/egg\" }` or `{ \"egg\": \"s3://my-bucket/egg\" }`. If S3 is used, make sure the cluster has read access on the library. You may need to launch the cluster with an instance profile to access the S3 URI.
    #[serde(rename = "egg", skip_serializing_if = "Option::is_none")]
    pub egg: Option<String>,
    /// If whl, URI of the wheel or zipped wheels to be installed. DBFS and S3 URIs are supported. For example: `{ \"whl\": \"dbfs:/my/whl\" }` or `{ \"whl\": \"s3://my-bucket/whl\" }`. If S3 is used, make sure the cluster has read access on the library. You may need to launch the cluster with an instance profile to access the S3 URI. Also the wheel file name needs to use the [correct convention](https://www.python.org/dev/peps/pep-0427/#file-format). If zipped wheels are to be installed, the file name suffix should be `.wheelhouse.zip`.
    #[serde(rename = "whl", skip_serializing_if = "Option::is_none")]
    pub whl: Option<String>,
    #[serde(rename = "pypi", skip_serializing_if = "Option::is_none")]
    pub pypi: Option<Box<crate::models::PythonPyPiLibrary>>,
    #[serde(rename = "maven", skip_serializing_if = "Option::is_none")]
    pub maven: Option<Box<crate::models::MavenLibrary>>,
    #[serde(rename = "cran", skip_serializing_if = "Option::is_none")]
    pub cran: Option<Box<crate::models::RCranLibrary>>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            jar: None,
            egg: None,
            whl: None,
            pypi: None,
            maven: None,
            cran: None,
        }
    }
}


