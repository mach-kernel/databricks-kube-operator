# MlDeleteRuns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_id** | **String** | The ID of the experiment containing the runs to delete. | 
**max_runs** | Option<**i32**> | An optional positive integer indicating the maximum number of runs to delete. The maximum allowed value for max_runs is 10000.  | [optional][default to 10000]
**max_timestamp_millis** | **i64** | The maximum creation timestamp in milliseconds since the UNIX epoch for deleting runs. Only runs created prior to or at this timestamp are deleted.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


