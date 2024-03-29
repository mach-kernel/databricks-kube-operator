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
pub struct ClusterInfo {
    /// If num_workers, number of worker nodes that this cluster must have. A cluster has one Spark driver and num_workers executors for a total of num_workers + 1 Spark nodes. **Note:** When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual number of workers. For instance, if a cluster is resized from 5 to 10 workers, this field is immediately updated to reflect the target size of 10 workers, whereas the workers listed in `executors` gradually increase from 5 to 10 as the new nodes are provisioned.
    #[serde(rename = "num_workers", skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(rename = "autoscale", skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Box<crate::models::AutoScale>>,
    /// Canonical identifier for the cluster. This ID is retained during cluster restarts and resizes, while each new cluster has a globally unique ID.
    #[serde(rename = "cluster_id", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// Creator user name. The field won’t be included in the response if the user has already been deleted.
    #[serde(rename = "creator_user_name", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<Box<crate::models::SparkNode>>,
    /// Nodes on which the Spark executors reside.
    #[serde(rename = "executors", skip_serializing_if = "Option::is_none")]
    pub executors: Option<Vec<crate::models::SparkNode>>,
    /// A canonical SparkContext identifier. This value _does_ change when the Spark driver restarts. The pair `(cluster_id, spark_context_id)` is a globally unique identifier over all Spark contexts.
    #[serde(rename = "spark_context_id", skip_serializing_if = "Option::is_none")]
    pub spark_context_id: Option<i64>,
    /// Port on which Spark JDBC server is listening in the driver node. No service listens on this port in executor nodes.
    #[serde(rename = "jdbc_port", skip_serializing_if = "Option::is_none")]
    pub jdbc_port: Option<i32>,
    /// Cluster name requested by the user. This doesn’t have to be unique. If not specified at creation, the cluster name is an empty string.
    #[serde(rename = "cluster_name", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// The runtime version of the cluster. You can retrieve a list of available runtime versions by using the [Runtime versions](https://docs.databricks.com/dev-tools/api/latest/clusters.html#runtime-versions) API call.
    #[serde(rename = "spark_version", skip_serializing_if = "Option::is_none")]
    pub spark_version: Option<String>,
    /// An arbitrary object where the object key is a configuration propery name and the value is a configuration property value.
    #[serde(rename = "spark_conf", skip_serializing_if = "Option::is_none")]
    pub spark_conf: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "aws_attributes", skip_serializing_if = "Option::is_none")]
    pub aws_attributes: Option<Box<crate::models::AwsAttributes>>,
    /// This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads. A list of available node types can be retrieved by using the [List node types](https://docs.databricks.com/dev-tools/api/latest/clusters.html#list-node-types) API call.
    #[serde(rename = "node_type_id", skip_serializing_if = "Option::is_none")]
    pub node_type_id: Option<String>,
    /// The node type of the Spark driver. This field is optional; if unset, the driver node type is set as the same value as `node_type_id` defined above.
    #[serde(rename = "driver_node_type_id", skip_serializing_if = "Option::is_none")]
    pub driver_node_type_id: Option<String>,
    /// SSH public key contents that are added to each Spark node in this cluster. The corresponding private keys can be used to login with the user name `ubuntu` on port `2200`. Up to 10 keys can be specified.
    #[serde(rename = "ssh_public_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<Vec<String>>,
    /// An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions>
    #[serde(rename = "custom_tags", skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "cluster_log_conf", skip_serializing_if = "Option::is_none")]
    pub cluster_log_conf: Option<Box<crate::models::ClusterLogConf>>,
    /// The configuration for storing init scripts. Any number of destinations can be specified. The scripts are executed sequentially in the order provided. If `cluster_log_conf` is specified, init script logs are sent to `<destination>/<cluster-ID>/init_scripts`.
    #[serde(rename = "init_scripts", skip_serializing_if = "Option::is_none")]
    pub init_scripts: Option<Vec<crate::models::InitScriptInfo>>,
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<Box<crate::models::DockerImage>>,
    /// An arbitrary object where the object key is an environment variable name and the value is an environment variable value.
    #[serde(rename = "spark_env_vars", skip_serializing_if = "Option::is_none")]
    pub spark_env_vars: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Automatically terminates the cluster after it is inactive for this time in minutes. If not set, this cluster is not be automatically terminated. If specified, the threshold must be between 10 and 10000 minutes. You can also set this value to 0 to explicitly disable automatic termination.
    #[serde(rename = "autotermination_minutes", skip_serializing_if = "Option::is_none")]
    pub autotermination_minutes: Option<i32>,
    /// Autoscaling Local Storage: when enabled, this cluster dynamically acquires additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly - refer to [Autoscaling local storage](https://docs.databricks.com/clusters/configure.html#autoscaling-local-storage) for details.
    #[serde(rename = "enable_elastic_disk", skip_serializing_if = "Option::is_none")]
    pub enable_elastic_disk: Option<bool>,
    /// The optional ID of the instance pool to which the cluster belongs. Refer to [Pools](https://docs.databricks.com/clusters/instance-pools/index.html) for details.
    #[serde(rename = "instance_pool_id", skip_serializing_if = "Option::is_none")]
    pub instance_pool_id: Option<String>,
    #[serde(rename = "cluster_source", skip_serializing_if = "Option::is_none")]
    pub cluster_source: Option<crate::models::ClusterSource>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::ClusterState>,
    /// A message associated with the most recent state transition (for example, the reason why the cluster entered a `TERMINATED` state). This field is unstructured, and its exact format is subject to change.
    #[serde(rename = "state_message", skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// Time (in epoch milliseconds) when the cluster creation request was received (when the cluster entered a `PENDING` state).
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Time (in epoch milliseconds) when the cluster was terminated, if applicable.
    #[serde(rename = "terminated_time", skip_serializing_if = "Option::is_none")]
    pub terminated_time: Option<i64>,
    /// Time when the cluster driver last lost its state (due to a restart or driver failure).
    #[serde(rename = "last_state_loss_time", skip_serializing_if = "Option::is_none")]
    pub last_state_loss_time: Option<i64>,
    /// Time (in epoch milliseconds) when the cluster was last active. A cluster is active if there is at least one command that has not finished on the cluster. This field is available after the cluster has reached a `RUNNING` state. Updates to this field are made as best-effort attempts. Certain versions of Spark do not support reporting of cluster activity. Refer to [Automatic termination](https://docs.databricks.com/clusters/clusters-manage.html#automatic-termination) for details.
    #[serde(rename = "last_activity_time", skip_serializing_if = "Option::is_none")]
    pub last_activity_time: Option<i64>,
    /// Total amount of cluster memory, in megabytes.
    #[serde(rename = "cluster_memory_mb", skip_serializing_if = "Option::is_none")]
    pub cluster_memory_mb: Option<i64>,
    /// Number of CPU cores available for this cluster. This can be fractional since certain node types are configured to share cores between Spark nodes on the same instance.
    #[serde(rename = "cluster_cores", skip_serializing_if = "Option::is_none")]
    pub cluster_cores: Option<f32>,
    /// An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions>
    #[serde(rename = "default_tags", skip_serializing_if = "Option::is_none")]
    pub default_tags: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "cluster_log_status", skip_serializing_if = "Option::is_none")]
    pub cluster_log_status: Option<Box<crate::models::LogSyncStatus>>,
    #[serde(rename = "termination_reason", skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<Box<crate::models::TerminationReason>>,
}

impl ClusterInfo {
    pub fn new() -> ClusterInfo {
        ClusterInfo {
            num_workers: None,
            autoscale: None,
            cluster_id: None,
            creator_user_name: None,
            driver: None,
            executors: None,
            spark_context_id: None,
            jdbc_port: None,
            cluster_name: None,
            spark_version: None,
            spark_conf: None,
            aws_attributes: None,
            node_type_id: None,
            driver_node_type_id: None,
            ssh_public_keys: None,
            custom_tags: None,
            cluster_log_conf: None,
            init_scripts: None,
            docker_image: None,
            spark_env_vars: None,
            autotermination_minutes: None,
            enable_elastic_disk: None,
            instance_pool_id: None,
            cluster_source: None,
            state: None,
            state_message: None,
            start_time: None,
            terminated_time: None,
            last_state_loss_time: None,
            last_activity_time: None,
            cluster_memory_mb: None,
            cluster_cores: None,
            default_tags: None,
            cluster_log_status: None,
            termination_reason: None,
        }
    }
}


