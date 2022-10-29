# ClusterAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_name** | Option<**String**> | Cluster name requested by the user. This doesn’t have to be unique. If not specified at creation, the cluster name is an empty string. | [optional]
**spark_version** | Option<**String**> | The runtime version of the cluster, for example “5.0.x-scala2.11”. You can retrieve a list of available runtime versions by using the [Runtime versions](https://docs.databricks.com/dev-tools/api/latest/clusters.html#runtime-versions) API call. | [optional]
**spark_conf** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary object where the object key is a configuration propery name and the value is a configuration property value. | [optional]
**aws_attributes** | Option<[**crate::models::AwsAttributes**](AwsAttributes.md)> |  | [optional]
**node_type_id** | Option<**String**> | This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads A list of available node types can be retrieved by using the [List node types](https://docs.databricks.com/dev-tools/api/latest/clusters.html#list-node-types) API call. | [optional]
**driver_node_type_id** | Option<**String**> | The node type of the Spark driver. This field is optional; if unset, the driver node type is set as the same value as `node_type_id` defined above. | [optional]
**ssh_public_keys** | Option<**Vec<String>**> | SSH public key contents that is added to each Spark node in this cluster. The corresponding private keys can be used to login with the user name `ubuntu` on port `2200`. Up to 10 keys can be specified. | [optional]
**custom_tags** | Option<**::std::collections::HashMap<String, String>**> | An object with key value pairs. The key length must be between 1 and 127 UTF-8 characters, inclusive. The value length must be less than or equal to 255 UTF-8 characters. For a list of all restrictions, see AWS Tag Restrictions: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions> | [optional]
**cluster_log_conf** | Option<[**crate::models::ClusterLogConf**](ClusterLogConf.md)> |  | [optional]
**init_scripts** | Option<[**Vec<crate::models::InitScriptInfo>**](InitScriptInfo.md)> | The configuration for storing init scripts. Any number of destinations can be specified. The scripts are executed sequentially in the order provided. If `cluster_log_conf` is specified, init script logs are sent to `<destination>/<cluster-ID>/init_scripts`. | [optional]
**docker_image** | Option<[**crate::models::DockerImage**](DockerImage.md)> |  | [optional]
**spark_env_vars** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An arbitrary object where the object key is an environment variable name and the value is an environment variable value. | [optional]
**autotermination_minutes** | Option<**i32**> | Automatically terminates the cluster after it is inactive for this time in minutes. If not set, this cluster is not be automatically terminated. If specified, the threshold must be between 10 and 10000 minutes. You can also set this value to 0 to explicitly disable automatic termination. | [optional]
**enable_elastic_disk** | Option<**bool**> | Autoscaling Local Storage: when enabled, this cluster dynamically acquires additional disk space when its Spark workers are running low on disk space. This feature requires specific AWS permissions to function correctly. Refer to [Autoscaling local storage](https://docs.databricks.com/clusters/configure.html#autoscaling-local-storage) for details. | [optional]
**instance_pool_id** | Option<**String**> | The optional ID of the instance pool to which the cluster belongs. Refer to [Pools](https://docs.databricks.com/clusters/instance-pools/index.html) for details. | [optional]
**cluster_source** | Option<[**crate::models::ClusterSource**](ClusterSource.md)> |  | [optional]
**policy_id** | Option<**String**> | A [cluster policy](https://docs.databricks.com/dev-tools/api/latest/policies.html) ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


