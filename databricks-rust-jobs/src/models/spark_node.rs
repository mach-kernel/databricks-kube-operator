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
pub struct SparkNode {
    /// Private IP address (typically a 10.x.x.x address) of the Spark node. This is different from the private IP address of the host instance.
    #[serde(rename = "private_ip", skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// Public DNS address of this node. This address can be used to access the Spark JDBC server on the driver node. To communicate with the JDBC server, traffic must be manually authorized by adding security group rules to the “worker-unmanaged” security group via the AWS console.
    #[serde(rename = "public_dns", skip_serializing_if = "Option::is_none")]
    pub public_dns: Option<String>,
    /// Globally unique identifier for this node.
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Globally unique identifier for the host instance from the cloud provider.
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// The timestamp (in millisecond) when the Spark node is launched.
    #[serde(rename = "start_timestamp", skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    #[serde(
        rename = "node_aws_attributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_aws_attributes: Option<Box<crate::models::SparkNodeAwsAttributes>>,
    /// The private IP address of the host instance.
    #[serde(rename = "host_private_ip", skip_serializing_if = "Option::is_none")]
    pub host_private_ip: Option<String>,
}

impl SparkNode {
    pub fn new() -> SparkNode {
        SparkNode {
            private_ip: None,
            public_dns: None,
            node_id: None,
            instance_id: None,
            start_timestamp: None,
            node_aws_attributes: None,
            host_private_ip: None,
        }
    }
}