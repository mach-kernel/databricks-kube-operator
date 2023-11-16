# ComputeGetInstancePool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disk_spec** | Option<[**crate::models::ComputeDiskSpec**](ComputeDiskSpec.md)> | Defines the specification of the disks that will be attached to all spark containers. | [optional]
**enable_elastic_disk** | Option<**bool**> | Autoscaling Local Storage: when enabled, this instances in this pool will dynamically acquire additional disk space when its Spark workers are running low on disk space. In AWS, this feature requires specific AWS permissions to function correctly - refer to the User Guide for more details. | [optional]
**custom_tags** | Option<**::std::collections::HashMap<String, String>**> | Additional tags for pool resources. Databricks will tag all pool resources (Eg., AWS instances and EBS volumes) with these tags in addition to `default_tags`. Notes:  - Currently, Databricks allows at most 45 custom tags | [optional]
**azure_attributes** | Option<[**crate::models::ComputeInstancePoolAzureAttributes**](ComputeInstancePoolAzureAttributes.md)> | Attributes related to instance pools running on Azure. If not specified at pool creation, a set of default values will be used. | [optional]
**max_capacity** | Option<**i32**> | Maximum number of outstanding instances to keep in the pool, including both instances used by clusters and idle instances. Clusters that require further instance provisioning will fail during upsize requests. | [optional]
**preloaded_spark_versions** | Option<**Vec<String>**> |  | [optional]
**default_tags** | Option<**::std::collections::HashMap<String, String>**> | Tags that are added by Databricks regardless of any `custom_tags`, including:    - Vendor: Databricks    - InstancePoolCreator: <user_id_of_creator>    - InstancePoolName: <name_of_pool>    - InstancePoolId: <id_of_pool> | [optional]
**aws_attributes** | Option<[**crate::models::ComputeInstancePoolAwsAttributes**](ComputeInstancePoolAwsAttributes.md)> | Attributes related to instance pools running on Amazon Web Services. If not specified at pool creation, a set of default values will be used. | [optional]
**status** | Option<[**crate::models::ComputeInstancePoolStatus**](ComputeInstancePoolStatus.md)> | Status of failed pending instances in the pool. | [optional]
**instance_pool_name** | Option<**String**> | Pool name requested by the user. Pool name must be unique. Length must be between 1 and 100 characters. | [optional]
**gcp_attributes** | Option<[**crate::models::ComputeInstancePoolGcpAttributes**](ComputeInstancePoolGcpAttributes.md)> | Attributes related to instance pools running on Google Cloud Platform. If not specified at pool creation, a set of default values will be used. | [optional]
**state** | Option<[**crate::models::ComputeInstancePoolState**](ComputeInstancePoolState.md)> |  | [optional]
**instance_pool_id** | **String** | Canonical unique identifier for the pool. | 
**min_idle_instances** | Option<**i32**> | Minimum number of idle instances to keep in the instance pool | [optional]
**preloaded_docker_images** | Option<[**Vec<crate::models::ComputeDockerImage>**](ComputeDockerImage.md)> |  | [optional]
**node_type_id** | Option<**String**> | This field encodes, through a single value, the resources available to each of the Spark nodes in this cluster. For example, the Spark nodes can be provisioned and optimized for memory or compute intensive workloads. A list of available node types can be retrieved by using the :method:clusters/listNodeTypes API call.  | [optional]
**stats** | Option<[**crate::models::ComputeInstancePoolStats**](ComputeInstancePoolStats.md)> | Usage statistics about the instance pool. | [optional]
**idle_instance_autotermination_minutes** | Option<**i32**> | Automatically terminates the extra instances in the pool cache after they are inactive for this time in minutes if min_idle_instances requirement is already met. If not set, the extra pool instances will be automatically terminated after a default timeout. If specified, the threshold must be between 0 and 10000 minutes. Users can also set this value to 0 to instantly remove idle instances from the cache if min cache size could still hold. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


