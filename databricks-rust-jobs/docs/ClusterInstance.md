# ClusterInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | Option<**String**> | The canonical identifier for the cluster used by a run. This field is always available for runs on existing clusters. For runs on new clusters, it becomes available once the cluster is created. This value can be used to view logs by browsing to `/#setting/sparkui/$cluster_id/driver-logs`. The logs continue to be available after the run completes.  The response won’t include this field if the identifier is not available yet. | [optional]
**spark_context_id** | Option<**String**> | The canonical identifier for the Spark context used by a run. This field is filled in once the run begins execution. This value can be used to view the Spark UI by browsing to `/#setting/sparkui/$cluster_id/$spark_context_id`. The Spark UI continues to be available after the run has completed.  The response won’t include this field if the identifier is not available yet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


