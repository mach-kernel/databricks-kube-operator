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
pub struct JobsClusterInstance {
    /// The canonical identifier for the cluster used by a run. This field is always available for runs on existing clusters. For runs on new clusters, it becomes available once the cluster is created. This value can be used to view logs by browsing to `/#setting/sparkui/$cluster_id/driver-logs`. The logs continue to be available after the run completes.  The response won’t include this field if the identifier is not available yet.
    #[serde(rename = "cluster_id", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// The canonical identifier for the Spark context used by a run. This field is filled in once the run begins execution. This value can be used to view the Spark UI by browsing to `/#setting/sparkui/$cluster_id/$spark_context_id`. The Spark UI continues to be available after the run has completed.  The response won’t include this field if the identifier is not available yet.
    #[serde(rename = "spark_context_id", skip_serializing_if = "Option::is_none")]
    pub spark_context_id: Option<String>,
}

impl JobsClusterInstance {
    pub fn new() -> JobsClusterInstance {
        JobsClusterInstance {
            cluster_id: None,
            spark_context_id: None,
        }
    }
}


