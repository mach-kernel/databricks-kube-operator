# ClusterInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num_workers** | Option<**i32**> | If num_workers, number of worker nodes that this cluster must have. A cluster has one Spark driver and num_workers executors for a total of num_workers + 1 Spark nodes. **Note:** When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual number of workers. For instance, if a cluster is resized from 5 to 10 workers, this field is immediately updated to reflect the target size of 10 workers, whereas the workers listed in `executors` gradually increase from 5 to 10 as the new nodes are provisioned. | [optional]
**autoscale** | Option<[**crate::models::AutoScale**](AutoScale.md)> |  | [optional]
**cluster_id** | Option<**String**> | Canonical identifier for the cluster. This ID is retained during cluster restarts and resizes, while each new cluster has a globally unique ID. | [optional]
**creator_user_name** | Option<**String**> | Creator user name. The field won’t be included in the response if the user has already been deleted. | [optional]
**driver** | Option<[**crate::models::SparkNode**](SparkNode.md)> |  | [optional]
**executors** | Option<[**Vec<crate::models::SparkNode>**](SparkNode.md)> | Nodes on which the Spark executors reside. | [optional]
**spark_context_id** | Option<**i64**> | A canonical SparkContext identifier. This value _does_ change when the Spark driver restarts. The pair `(cluster_id, spark_context_id)` is a globally unique identifier over all Spark contexts. | [optional]
**jdbc_port** | Option<**i32**> | Port on which Spark JDBC server is listening in the driver node. No service listens on this port in executor nodes. | [optional]
**cluster_name** | Option<**String**> | Cluster name requested by the user. This doesn’t have to be unique. If not specified at creation, the cluster name is an empty string. | [optional]
**spark_version** | Option<**String**> | The runtime version of the cluster. You can retrieve a list of available runtime versions by using the [Runtime versions](https://docs.databricks.com/dev-tools/api/latest/clusters.html#runtime-versions) API call. | [optional]
**spark_conf** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary object where the object key is a configuration propery name and the value is a configuration property value. | [optional]
**aws_attributes** | Option<[**crate::models::AwsAttributes**](AwsAttributes.md)> |  | [optional]
**node_type_id** | Option<**String**> | This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads. A list of available node types can be retrieved by using the [List node types](https://docs.databricks.com/dev-tools/api/latest/clusters.html#list-node-types) API call. | [optional]
**driver_node_type_id** | Option<**String**> | The node type of the Spark driver. This field is optional; if unset, the driver node type is set as the same value as `node_type_id` defined above. | [optional]
**ssh_public_keys** | Option<**Vec<String>**> | SSH public key contents that are added to each Spark node in this cluster. The corresponding private keys can be used to login with the user name `ubuntu` on port `2200`. Up to 10 keys can be specified. | [optional]
**custom_tags** | Option<**::std::collections::HashMap<String, String>**> | An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions> | [optional]
**cluster_log_conf** | Option<[**crate::models::ClusterLogConf**](ClusterLogConf.md)> |  | [optional]
**init_scripts** | Option<[**Vec<crate::models::InitScriptInfo>**](InitScriptInfo.md)> | The configuration for storing init scripts. Any number of destinations can be specified. The scripts are executed sequentially in the order provided. If `cluster_log_conf` is specified, init script logs are sent to `<destination>/<cluster-ID>/init_scripts`. | [optional]
**docker_image** | Option<[**crate::models::DockerImage**](DockerImage.md)> |  | [optional]
**spark_env_vars** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary object where the object key is an environment variable name and the value is an environment variable value. | [optional]
**autotermination_minutes** | Option<**i32**> | Automatically terminates the cluster after it is inactive for this time in minutes. If not set, this cluster is not be automatically terminated. If specified, the threshold must be between 10 and 10000 minutes. You can also set this value to 0 to explicitly disable automatic termination. | [optional]
**enable_elastic_disk** | Option<**bool**> | Autoscaling Local Storage: when enabled, this cluster dynamically acquires additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly - refer to [Autoscaling local storage](https://docs.databricks.com/clusters/configure.html#autoscaling-local-storage) for details. | [optional]
**instance_pool_id** | Option<**String**> | The optional ID of the instance pool to which the cluster belongs. Refer to [Pools](https://docs.databricks.com/clusters/instance-pools/index.html) for details. | [optional]
**cluster_source** | Option<[**crate::models::ClusterSource**](ClusterSource.md)> |  | [optional]
**state** | Option<[**crate::models::ClusterState**](ClusterState.md)> |  | [optional]
**state_message** | Option<**String**> | A message associated with the most recent state transition (for example, the reason why the cluster entered a `TERMINATED` state). This field is unstructured, and its exact format is subject to change. | [optional]
**start_time** | Option<**i64**> | Time (in epoch milliseconds) when the cluster creation request was received (when the cluster entered a `PENDING` state). | [optional]
**terminated_time** | Option<**i64**> | Time (in epoch milliseconds) when the cluster was terminated, if applicable. | [optional]
**last_state_loss_time** | Option<**i64**> | Time when the cluster driver last lost its state (due to a restart or driver failure). | [optional]
**last_activity_time** | Option<**i64**> | Time (in epoch milliseconds) when the cluster was last active. A cluster is active if there is at least one command that has not finished on the cluster. This field is available after the cluster has reached a `RUNNING` state. Updates to this field are made as best-effort attempts. Certain versions of Spark do not support reporting of cluster activity. Refer to [Automatic termination](https://docs.databricks.com/clusters/clusters-manage.html#automatic-termination) for details. | [optional]
**cluster_memory_mb** | Option<**i64**> | Total amount of cluster memory, in megabytes. | [optional]
**cluster_cores** | Option<**f32**> | Number of CPU cores available for this cluster. This can be fractional since certain node types are configured to share cores between Spark nodes on the same instance. | [optional]
**default_tags** | Option<**::std::collections::HashMap<String, String>**> | An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions> | [optional]
**cluster_log_status** | Option<[**crate::models::LogSyncStatus**](LogSyncStatus.md)> |  | [optional]
**termination_reason** | Option<[**crate::models::TerminationReason**](TerminationReason.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


