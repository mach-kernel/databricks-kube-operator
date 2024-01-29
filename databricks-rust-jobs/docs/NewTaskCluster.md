# NewTaskCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num_workers** | Option<**i32**> | If num_workers, number of worker nodes that this cluster must have. A cluster has one Spark driver and num_workers executors for a total of num_workers + 1 Spark nodes. When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual current number of workers. For example, if a cluster is resized from 5 to 10 workers, this field immediately updates to reflect the target size of 10 workers, whereas the workers listed in `spark_info` gradually increase from 5 to 10 as the new nodes are provisioned. | [optional]
**autoscale** | Option<[**crate::models::AutoScale**](AutoScale.md)> |  | [optional]
**spark_version** | **String** | The Spark version of the cluster. A list of available Spark versions can be retrieved by using the [Runtime versions](https://docs.databricks.com/dev-tools/api/latest/clusters.html#runtime-versions) API call. | 
**spark_conf** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary object where the object key is a configuration propery name and the value is a configuration property value. | [optional]
**aws_attributes** | Option<[**crate::models::AwsAttributes**](AwsAttributes.md)> |  | [optional]
**node_type_id** | Option<**String**> | This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads A list of available node types can be retrieved by using the [List node types](https://docs.databricks.com/dev-tools/api/latest/clusters.html#list-node-types) API call. | [optional]
**data_security_mode** | Option<**String**> | Data security mode decides what data governance model to use when accessing data from a cluster. | [optional]
**single_user_name** | Option<**String**> | Single user name if data_security_mode is SINGLE_USER | [optional]
**driver_node_type_id** | Option<**String**> | The node type of the Spark driver. This field is optional; if unset, the driver node type is set as the same value as `node_type_id` defined above. | [optional]
**ssh_public_keys** | Option<**Vec<String>**> | SSH public key contents that are added to each Spark node in this cluster. The corresponding private keys can be used to login with the user name `ubuntu` on port `2200`. Up to 10 keys can be specified. | [optional]
**custom_tags** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions> | [optional]
**cluster_log_conf** | Option<[**crate::models::ClusterLogConf**](ClusterLogConf.md)> |  | [optional]
**init_scripts** | Option<[**Vec<crate::models::InitScriptInfo>**](InitScriptInfo.md)> | The configuration for storing init scripts. Any number of scripts can be specified. The scripts are executed sequentially in the order provided. If `cluster_log_conf` is specified, init script logs are sent to `<destination>/<cluster-id>/init_scripts`. | [optional]
**spark_env_vars** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary object where the object key is an environment variable name and the value is an environment variable value. | [optional]
**enable_elastic_disk** | Option<**bool**> | Autoscaling Local Storage: when enabled, this cluster dynamically acquires additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly - refer to [Autoscaling local storage](https://docs.databricks.com/clusters/configure.html#autoscaling-local-storage) for details. | [optional]
**driver_instance_pool_id** | Option<**String**> | The optional ID of the instance pool to use for the driver node. You must also specify `instance_pool_id`. Refer to [Instance Pools API](https://docs.databricks.com/dev-tools/api/latest/instance-pools.html) for details. | [optional]
**instance_pool_id** | Option<**String**> | The optional ID of the instance pool to use for cluster nodes. If `driver_instance_pool_id` is present, `instance_pool_id` is used for worker nodes only. Otherwise, it is used for both the driver node and worker nodes. Refer to [Instance Pools API](https://docs.databricks.com/dev-tools/api/latest/instance-pools.html) for details. | [optional]
**policy_id** | Option<**String**> | A [cluster policy](https://docs.databricks.com/dev-tools/api/latest/policies.html) ID. Either `node_type_id` or `instance_pool_id` must be specified in the cluster policy if they are not specified in this job cluster object. | [optional]
**enable_local_disk_encryption** | Option<**bool**> | Determines whether encryption of disks locally attached to the cluster is enabled. | [optional]
**docker_image** | Option<[**crate::models::DockerImage**](DockerImage.md)> |  | [optional]
**runtime_engine** | Option<**String**> | The type of runtime engine to use. If not specified, the runtime engine type is inferred based on the `spark_version` value. Allowed values include:  * `PHOTON`: Use the Photon runtime engine type. * `STANDARD`: Use the standard runtime engine type.  This field is optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


