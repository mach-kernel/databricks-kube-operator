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
pub struct ClusterSize {
    /// If num_workers, number of worker nodes that this cluster must have. A cluster has one Spark driver and num_workers executors for a total of num_workers + 1 Spark nodes. When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual number of workers. For instance, if a cluster is resized from 5 to 10 workers, this field is updated to reflect the target size of 10 workers, whereas the workers listed in executors gradually increase from 5 to 10 as the new nodes are provisioned.
    #[serde(rename = "num_workers", skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(rename = "autoscale", skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Box<crate::models::AutoScale>>,
}

impl ClusterSize {
    pub fn new() -> ClusterSize {
        ClusterSize {
            num_workers: None,
            autoscale: None,
        }
    }
}


