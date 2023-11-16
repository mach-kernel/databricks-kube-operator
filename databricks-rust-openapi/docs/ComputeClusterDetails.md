# ComputeClusterDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creator_user_name** | Option<**String**> | Creator user name. The field won't be included in the response if the user has already been deleted. | [optional]
**state_message** | Option<**String**> | A message associated with the most recent state transition (Eg., the reason why the cluster entered a `TERMINATED` state). | [optional]
**enable_elastic_disk** | Option<**bool**> | Autoscaling Local Storage: when enabled, this cluster will dynamically acquire additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly - refer to the User Guide for more details. | [optional]
**cluster_cores** | Option<**f64**> | Number of CPU cores available for this cluster. Note that this can be fractional, Eg. 7.5 cores, since certain node types are configured to share cores between Spark nodes on the same instance. | [optional]
**ssh_public_keys** | Option<**Vec<String>**> |  | [optional]
**custom_tags** | Option<**::std::collections::HashMap<String, String>**> | Additional tags for cluster resources. Databricks will tag all cluster resources (Eg., AWS instances and EBS volumes) with these tags in addition to `default_tags`. Notes:  - Currently, Databricks allows at most 45 custom tags  - Clusters can only reuse cloud resources if the resources' tags are a subset of the cluster tags | [optional]
**azure_attributes** | Option<[**crate::models::ComputeAzureAttributes**](ComputeAzureAttributes.md)> | Attributes related to clusters running on Microsoft Azure. If not specified at cluster creation, a set of default values will be used. | [optional]
**driver** | Option<[**crate::models::ComputeSparkNode**](ComputeSparkNode.md)> | Node on which the Spark driver resides. The driver node contains the Spark master and the <Databricks> application that manages the per-notebook Spark REPLs. | [optional]
**termination_reason** | Option<[**crate::models::ComputeTerminationReason**](ComputeTerminationReason.md)> | Information about why the cluster was terminated. This field only appears when the cluster is in a `TERMINATING` or `TERMINATED` state. | [optional]
**init_scripts** | Option<[**Vec<crate::models::ComputeInitScriptInfo>**](ComputeInitScriptInfo.md)> |  | [optional]
**spark_version** | Option<**String**> | The Spark version of the cluster, Eg. `3.3.x-scala2.11`. A list of available Spark versions can be retrieved by using the :method:clusters/sparkVersions API call.  | [optional]
**driver_instance_pool_id** | Option<**String**> | The optional ID of the instance pool for the driver of the cluster belongs. The pool cluster uses the instance pool with id (instance_pool_id) if the driver pool is not assigned. | [optional]
**data_security_mode** | Option<[**crate::models::ComputeDataSecurityMode**](ComputeDataSecurityMode.md)> |  | [optional]
**default_tags** | Option<**::std::collections::HashMap<String, String>**> | Tags that are added by Databricks regardless of any `custom_tags`, including:    - Vendor: Databricks    - Creator: <username_of_creator>    - ClusterName: <name_of_cluster>    - ClusterId: <id_of_cluster>    - Name: <Databricks internal use> | [optional]
**executors** | Option<[**Vec<crate::models::ComputeSparkNode>**](ComputeSparkNode.md)> |  | [optional]
**autoscale** | Option<[**crate::models::ComputeAutoScale**](ComputeAutoScale.md)> | Parameters needed in order to automatically scale clusters up and down based on load. Note: autoscaling works best with DB runtime versions 3.0 or later. | [optional]
**cluster_source** | Option<[**crate::models::ComputeClusterSource**](ComputeClusterSource.md)> |  | [optional]
**driver_node_type_id** | Option<**String**> | The node type of the Spark driver. Note that this field is optional; if unset, the driver node type will be set as the same value as `node_type_id` defined above.  | [optional]
**policy_id** | Option<**String**> | The ID of the cluster policy used to create the cluster if applicable. | [optional]
**cluster_log_status** | Option<[**crate::models::ComputeLogSyncStatus**](ComputeLogSyncStatus.md)> | Cluster log delivery status. | [optional]
**aws_attributes** | Option<[**crate::models::ComputeAwsAttributes**](ComputeAwsAttributes.md)> | Attributes related to clusters running on Amazon Web Services. If not specified at cluster creation, a set of default values will be used. | [optional]
**enable_local_disk_encryption** | Option<**bool**> | Whether to enable LUKS on cluster VMs' local disks | [optional]
**spark_conf** | Option<**::std::collections::HashMap<String, String>**> | An object containing a set of optional, user-specified Spark configuration key-value pairs. Users can also pass in a string of extra JVM options to the driver and the executors via `Sparkdriver.extraJavaOptions` and `spark.executor.extraJavaOptions` respectively.  | [optional]
**jdbc_port** | Option<**i32**> | Port on which Spark JDBC server is listening, in the driver nod. No service will be listeningon on this port in executor nodes. | [optional]
**num_workers** | Option<**i32**> | Number of worker nodes that this cluster should have. A cluster has one Spark Driver and `num_workers` Executors for a total of `num_workers` + 1 Spark nodes.  Note: When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual current number of workers. For instance, if a cluster is resized from 5 to 10 workers, this field will immediately be updated to reflect the target size of 10 workers, whereas the workers listed in `spark_info` will gradually increase from 5 to 10 as the new nodes are provisioned. | [optional]
**docker_image** | Option<[**crate::models::ComputeDockerImage**](ComputeDockerImage.md)> |  | [optional]
**gcp_attributes** | Option<[**crate::models::ComputeGcpAttributes**](ComputeGcpAttributes.md)> | Attributes related to clusters running on Google Cloud Platform. If not specified at cluster creation, a set of default values will be used. | [optional]
**spark_context_id** | Option<**i64**> | A canonical SparkContext identifier. This value *does* change when the Spark driver restarts. The pair `(cluster_id, spark_context_id)` is a globally unique identifier over all Spark contexts. | [optional]
**cluster_memory_mb** | Option<**i64**> | Total amount of cluster memory, in megabytes | [optional]
**autotermination_minutes** | Option<**i32**> | Automatically terminates the cluster after it is inactive for this time in minutes. If not set, this cluster will not be automatically terminated. If specified, the threshold must be between 10 and 10000 minutes. Users can also set this value to 0 to explicitly disable automatic termination. | [optional]
**cluster_id** | Option<**String**> | Canonical identifier for the cluster. This id is retained during cluster restarts and resizes, while each new cluster has a globally unique id. | [optional]
**state** | Option<[**crate::models::ComputeState**](ComputeState.md)> | Current state of the cluster. | [optional]
**workload_type** | Option<[**crate::models::ComputeWorkloadType**](ComputeWorkloadType.md)> |  | [optional]
**last_restarted_time** | Option<**i64**> | the timestamp that the cluster was started/restarted | [optional]
**instance_pool_id** | Option<**String**> | The optional ID of the instance pool to which the cluster belongs. | [optional]
**last_state_loss_time** | Option<**i64**> | Time when the cluster driver last lost its state (due to a restart or driver failure). | [optional]
**single_user_name** | Option<**String**> | Single user name if data_security_mode is `SINGLE_USER` | [optional]
**cluster_name** | Option<**String**> | Cluster name requested by the user. This doesn't have to be unique. If not specified at creation, the cluster name will be an empty string.  | [optional]
**spark_env_vars** | Option<**::std::collections::HashMap<String, String>**> | An object containing a set of optional, user-specified environment variable key-value pairs. Please note that key-value pair of the form (X,Y) will be exported as is (Ie., `export X='Y'`) while launching the driver and workers.  In order to specify an additional set of `SPARK_DAEMON_JAVA_OPTS`, we recommend appending them to `$SPARK_DAEMON_JAVA_OPTS` as shown in the example below. This ensures that all default databricks managed environmental variables are included as well.  Example Spark environment variables: `{\"SPARK_WORKER_MEMORY\": \"28000m\", \"SPARK_LOCAL_DIRS\": \"/local_disk0\"}` or `{\"SPARK_DAEMON_JAVA_OPTS\": \"$SPARK_DAEMON_JAVA_OPTS -Dspark.shuffle.service.enabled=true\"}` | [optional]
**start_time** | Option<**i64**> | Time (in epoch milliseconds) when the cluster creation request was received (when the cluster entered a `PENDING` state). | [optional]
**node_type_id** | Option<**String**> | This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads. A list of available node types can be retrieved by using the :method:clusters/listNodeTypes API call.  | [optional]
**terminated_time** | Option<**i64**> | Time (in epoch milliseconds) when the cluster was terminated, if applicable. | [optional]
**cluster_log_conf** | Option<[**crate::models::ComputeClusterLogConf**](ComputeClusterLogConf.md)> | The configuration for delivering spark logs to a long-term storage destination. Two kinds of destinations (dbfs and s3) are supported. Only one destination can be specified for one cluster. If the conf is given, the logs will be delivered to the destination every `5 mins`. The destination of driver logs is `$destination/$clusterId/driver`, while the destination of executor logs is `$destination/$clusterId/executor`. | [optional]
**runtime_engine** | Option<[**crate::models::ComputeRuntimeEngine**](ComputeRuntimeEngine.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


