# RepairHistoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The repair history item type. Indicates whether a run is the original run or a repair run. | [optional]
**start_time** | Option<**i64**> | The start time of the (repaired) run. | [optional]
**end_time** | Option<**i64**> | The end time of the (repaired) run. | [optional]
**state** | Option<[**crate::models::RunState**](RunState.md)> |  | [optional]
**id** | Option<**i64**> | The ID of the repair. Only returned for the items that represent a repair in `repair_history`. | [optional]
**task_run_ids** | Option<**Vec<i64>**> | The run IDs of the task runs that ran as part of this repair history item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


