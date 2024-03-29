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
pub struct EventDetails {
    /// The number of nodes in the cluster.
    #[serde(rename = "current_num_workers", skip_serializing_if = "Option::is_none")]
    pub current_num_workers: Option<i32>,
    /// The targeted number of nodes in the cluster.
    #[serde(rename = "target_num_workers", skip_serializing_if = "Option::is_none")]
    pub target_num_workers: Option<i32>,
    #[serde(rename = "previous_attributes", skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<Box<crate::models::AwsAttributes>>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::AwsAttributes>>,
    #[serde(rename = "previous_cluster_size", skip_serializing_if = "Option::is_none")]
    pub previous_cluster_size: Option<Box<crate::models::ClusterSize>>,
    #[serde(rename = "cluster_size", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<Box<crate::models::ClusterSize>>,
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<crate::models::ResizeCause>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Box<crate::models::TerminationReason>>,
    /// The user that caused the event to occur. (Empty if it was done by Databricks.)
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl EventDetails {
    pub fn new() -> EventDetails {
        EventDetails {
            current_num_workers: None,
            target_num_workers: None,
            previous_attributes: None,
            attributes: None,
            previous_cluster_size: None,
            cluster_size: None,
            cause: None,
            reason: None,
            user: None,
        }
    }
}


