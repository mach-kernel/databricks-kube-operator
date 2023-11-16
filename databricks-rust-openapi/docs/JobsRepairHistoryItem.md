# JobsRepairHistoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_time** | Option<**i64**> | The end time of the (repaired) run. | [optional]
**id** | Option<**i64**> | The ID of the repair. Only returned for the items that represent a repair in `repair_history`. | [optional]
**start_time** | Option<**i64**> | The start time of the (repaired) run. | [optional]
**state** | Option<[**crate::models::JobsRunState**](JobsRunState.md)> |  | [optional]
**task_run_ids** | Option<**Vec<i64>**> |  | [optional]
**r#type** | Option<**String**> | The repair history item type. Indicates whether a run is the original run or a repair run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


