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
pub struct NewCluster {
    /// If num_workers, number of worker nodes that this cluster must have. A cluster has one Spark driver and num_workers executors for a total of num_workers + 1 Spark nodes. When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual current number of workers. For example, if a cluster is resized from 5 to 10 workers, this field immediately updates to reflect the target size of 10 workers, whereas the workers listed in `spark_info` gradually increase from 5 to 10 as the new nodes are provisioned.
    #[serde(rename = "num_workers", skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(rename = "autoscale", skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Box<crate::models::AutoScale>>,
    /// The Spark version of the cluster. A list of available Spark versions can be retrieved by using the [Runtime versions](https://docs.databricks.com/dev-tools/api/latest/clusters.html#runtime-versions) API call.
    #[serde(rename = "spark_version")]
    pub spark_version: String,
    /// An arbitrary object where the object key is a configuration propery name and the value is a configuration property value.
    #[serde(rename = "spark_conf", skip_serializing_if = "Option::is_none")]
    pub spark_conf: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "aws_attributes", skip_serializing_if = "Option::is_none")]
    pub aws_attributes: Option<Box<crate::models::AwsAttributes>>,
    /// This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads A list of available node types can be retrieved by using the [List node types](https://docs.databricks.com/dev-tools/api/latest/clusters.html#list-node-types) API call.
    #[serde(rename = "node_type_id", skip_serializing_if = "Option::is_none")]
    pub node_type_id: Option<String>,
    /// The node type of the Spark driver. This field is optional; if unset, the driver node type is set as the same value as `node_type_id` defined above.
    #[serde(
        rename = "driver_node_type_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub driver_node_type_id: Option<String>,
    /// SSH public key contents that are added to each Spark node in this cluster. The corresponding private keys can be used to login with the user name `ubuntu` on port `2200`. Up to 10 keys can be specified.
    #[serde(rename = "ssh_public_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<Vec<String>>,
    /// An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions>
    #[serde(rename = "custom_tags", skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "cluster_log_conf", skip_serializing_if = "Option::is_none")]
    pub cluster_log_conf: Option<Box<crate::models::ClusterLogConf>>,
    /// The configuration for storing init scripts. Any number of scripts can be specified. The scripts are executed sequentially in the order provided. If `cluster_log_conf` is specified, init script logs are sent to `<destination>/<cluster-id>/init_scripts`.
    #[serde(rename = "init_scripts", skip_serializing_if = "Option::is_none")]
    pub init_scripts: Option<Vec<crate::models::InitScriptInfo>>,
    /// An arbitrary object where the object key is an environment variable name and the value is an environment variable value.
    #[serde(rename = "spark_env_vars", skip_serializing_if = "Option::is_none")]
    pub spark_env_vars: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Autoscaling Local Storage: when enabled, this cluster dynamically acquires additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly - refer to [Autoscaling local storage](https://docs.databricks.com/clusters/configure.html#autoscaling-local-storage) for details.
    #[serde(
        rename = "enable_elastic_disk",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_elastic_disk: Option<bool>,
    /// The optional ID of the instance pool to use for the driver node. You must also specify `instance_pool_id`. Refer to [Instance Pools API](https://docs.databricks.com/dev-tools/api/latest/instance-pools.html) for details.
    #[serde(
        rename = "driver_instance_pool_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub driver_instance_pool_id: Option<String>,
    /// The optional ID of the instance pool to use for cluster nodes. If `driver_instance_pool_id` is present, `instance_pool_id` is used for worker nodes only. Otherwise, it is used for both the driver node and worker nodes. Refer to [Instance Pools API](https://docs.databricks.com/dev-tools/api/latest/instance-pools.html) for details.
    #[serde(rename = "instance_pool_id", skip_serializing_if = "Option::is_none")]
    pub instance_pool_id: Option<String>,
    /// A [cluster policy](https://docs.databricks.com/dev-tools/api/latest/policies.html) ID. Either `node_type_id` or `instance_pool_id` must be specified in the cluster policy if they are not specified in this job cluster object.
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// Determines whether encryption of disks locally attached to the cluster is enabled.
    #[serde(
        rename = "enable_local_disk_encryption",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_local_disk_encryption: Option<bool>,
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<Box<crate::models::DockerImage>>,
    /// The type of runtime engine to use. If not specified, the runtime engine type is inferred based on the `spark_version` value. Allowed values include:  * `PHOTON`: Use the Photon runtime engine type. * `STANDARD`: Use the standard runtime engine type.  This field is optional.
    #[serde(rename = "runtime_engine", skip_serializing_if = "Option::is_none")]
    pub runtime_engine: Option<String>,
}

impl NewCluster {
    pub fn new(spark_version: String) -> NewCluster {
        NewCluster {
            num_workers: None,
            autoscale: None,
            spark_version,
            spark_conf: None,
            aws_attributes: None,
            node_type_id: None,
            driver_node_type_id: None,
            ssh_public_keys: None,
            custom_tags: None,
            cluster_log_conf: None,
            init_scripts: None,
            spark_env_vars: None,
            enable_elastic_disk: None,
            driver_instance_pool_id: None,
            instance_pool_id: None,
            policy_id: None,
            enable_local_disk_encryption: None,
            docker_image: None,
            runtime_engine: None,
        }
    }
}
