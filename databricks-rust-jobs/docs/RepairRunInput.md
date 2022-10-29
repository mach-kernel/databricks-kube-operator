# RepairRunInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**i64**> | The job run ID of the run to repair. The run must not be in progress. | [optional]
**rerun_tasks** | Option<**Vec<String>**> | The task keys of the task runs to repair. | [optional]
**latest_repair_id** | Option<**i64**> | The ID of the latest repair. This parameter is not required when repairing a run for the first time, but must be provided on subsequent requests to repair the same run. | [optional]
**rerun_all_failed_tasks** | Option<**bool**> | If true, repair all failed tasks. Only one of rerun_tasks or rerun_all_failed_tasks can be used. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


