# SqlGetWarehouseResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_stop_mins** | Option<**i32**> | The amount of time in minutes that a SQL warehouse must be idle (Ie., no RUNNING queries) before it is automatically stopped.  Supported values:   - Must be == 0 or >= 10 mins   - 0 indicates no autostop.  Defaults to 120 mins | [optional][default to 120]
**warehouse_type** | Option<**String**> | Warehouse type: `PRO` or `CLASSIC`. If you want to use serverless compute, you must set to `PRO` and also set the field `enable_serverless_compute` to `true`. | [optional]
**jdbc_url** | Option<**String**> | the jdbc connection string for this warehouse | [optional]
**health** | Option<[**crate::models::SqlEndpointHealth**](SqlEndpointHealth.md)> | Optional health status. Assume the warehouse is healthy if this field is not set. | [optional]
**odbc_params** | Option<[**crate::models::SqlOdbcParams**](SqlOdbcParams.md)> | ODBC parameters for the SQL warehouse | [optional]
**instance_profile_arn** | Option<**String**> | Deprecated. Instance profile used to pass IAM role to the cluster | [optional]
**tags** | Option<[**crate::models::SqlEndpointTags**](SqlEndpointTags.md)> | A set of key-value pairs that will be tagged on all resources (Eg., AWS instances and EBS volumes) associated with this SQL warehouse.  Supported values:   - Number of tags < 45. | [optional]
**id** | Option<**String**> | unique identifier for warehouse | [optional]
**min_num_clusters** | Option<**i32**> | Minimum number of available clusters that will be maintained for this SQL warehouse. Increasing this will ensure that a larger number of clusters are always running and therefore may reduce the cold start time for new queries. This is similar to reserved vs. revocable cores in a resource manager.  Supported values:   - Must be > 0   - Must be <= min(max_num_clusters, 30)  Defaults to 1 | [optional][default to 1]
**enable_serverless_compute** | Option<**bool**> | Configures whether the warehouse should use serverless compute | [optional]
**max_num_clusters** | Option<**i32**> | Maximum number of clusters that the autoscaler will create to handle concurrent queries.  Supported values:   - Must be >= min_num_clusters   - Must be <= 30.  Defaults to min_clusters if unset. | [optional]
**name** | Option<**String**> | Logical name for the cluster.  Supported values:   - Must be unique within an org.   - Must be less than 100 characters. | [optional]
**enable_photon** | Option<**bool**> | Configures whether the warehouse should use Photon optimized clusters.  Defaults to false. | [optional]
**creator_name** | Option<**String**> | warehouse creator name | [optional]
**num_clusters** | Option<**i32**> | current number of clusters running for the service | [optional]
**cluster_size** | Option<**String**> | Size of the clusters allocated for this warehouse. Increasing the size of a spark cluster allows you to run larger queries on it. If you want to increase the number of concurrent queries, please tune max_num_clusters.  Supported values: - 2X-Small - X-Small - Small - Medium - Large - X-Large - 2X-Large - 3X-Large - 4X-Large | [optional]
**state** | Option<[**crate::models::SqlState**](SqlState.md)> |  | [optional]
**channel** | Option<[**crate::models::SqlChannel**](SqlChannel.md)> | Channel Details | [optional]
**spot_instance_policy** | Option<[**crate::models::SqlSpotInstancePolicy**](SqlSpotInstancePolicy.md)> |  | [optional]
**num_active_sessions** | Option<**i64**> | current number of active sessions for the warehouse | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


