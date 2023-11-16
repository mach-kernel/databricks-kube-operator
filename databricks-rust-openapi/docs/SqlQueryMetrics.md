# SqlQueryMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_time_ms** | Option<**i32**> | Time spent executing the query, in milliseconds. | [optional]
**query_execution_time_ms** | Option<**i32**> | Reserved for internal use. | [optional]
**photon_total_time_ms** | Option<**i32**> | Total execution time for all individual Photon query engine tasks in the query, in milliseconds. | [optional]
**pruned_files_count** | Option<**i32**> | Total number of files from all tables not read due to pruning | [optional]
**read_remote_bytes** | Option<**i32**> | Size of persistent data read from cloud object storage on your cloud tenant, in bytes. | [optional]
**planning_phases** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**overloading_queue_start_timestamp** | Option<**i32**> | Timestamp of when the query was enqueued waiting while the warehouse was at max load. This field is optional and will not appear if the query skipped the overloading queue. | [optional]
**read_files_count** | Option<**i32**> | Number of files read after pruning. | [optional]
**write_remote_bytes** | Option<**i32**> | Size pf persistent data written to cloud object storage in your cloud tenant, in bytes. | [optional]
**metadata_time_ms** | Option<**i32**> | Reserved for internal use. | [optional]
**result_from_cache** | Option<**bool**> | true if the query result was fetched from cache, false otherwise. | [optional]
**result_fetch_time_ms** | Option<**i32**> | Time spent fetching the query results after the execution finished, in milliseconds. | [optional]
**task_total_time_ms** | Option<**i32**> | Sum of execution time for all of the query’s tasks, in milliseconds. | [optional]
**rows_produced_count** | Option<**i32**> | Total number of rows returned by the query. | [optional]
**total_time_ms** | Option<**i32**> | Total execution time of the query from the client’s point of view, in milliseconds. | [optional]
**spill_to_disk_bytes** | Option<**i32**> | Size of data temporarily written to disk while executing the query, in bytes. | [optional]
**provisioning_queue_start_timestamp** | Option<**i32**> | Timestamp of when the query was enqueued waiting for a cluster to be provisioned for the warehouse. This field is optional and will not appear if the query skipped the provisioning queue. | [optional]
**pruned_bytes** | Option<**i32**> | Total number of bytes in all tables not read due to pruning | [optional]
**read_cache_bytes** | Option<**i32**> | Size of persistent data read from the cache, in bytes. | [optional]
**planning_time_ms** | Option<**i32**> | Reserved for internal use. | [optional]
**network_sent_bytes** | Option<**i32**> | Total amount of data sent over the network between executor nodes during shuffle, in bytes. | [optional]
**rows_read_count** | Option<**i32**> | Total number of rows read by the query. | [optional]
**read_bytes** | Option<**i32**> | Total size of data read by the query, in bytes. | [optional]
**query_compilation_start_timestamp** | Option<**i32**> | Timestamp of when the underlying compute started compilation of the query. | [optional]
**compilation_time_ms** | Option<**i32**> | Time spent loading metadata and optimizing the query, in milliseconds. | [optional]
**read_partitions_count** | Option<**i32**> | Number of partitions read after pruning. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


