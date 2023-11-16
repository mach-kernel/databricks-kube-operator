# MlRestoreRuns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_id** | **String** | The ID of the experiment containing the runs to restore. | 
**max_runs** | Option<**i32**> | An optional positive integer indicating the maximum number of runs to restore. The maximum allowed value for max_runs is 10000.  | [optional][default to 10000]
**min_timestamp_millis** | **i64** | The minimum deletion timestamp in milliseconds since the UNIX epoch for restoring runs. Only runs deleted no earlier than this timestamp are restored.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


