# ComputeGetEvents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | **String** | The ID of the cluster to retrieve events about. | 
**end_time** | Option<**i64**> | The end time in epoch milliseconds. If empty, returns events up to the current time. | [optional]
**event_types** | Option<[**Vec<crate::models::ComputeEventType>**](ComputeEventType.md)> |  | [optional]
**limit** | Option<**i64**> | The maximum number of events to include in a page of events. Defaults to 50, and maximum allowed value is 500. | [optional][default to 50]
**offset** | Option<**i64**> | The offset in the result set. Defaults to 0 (no offset). When an offset is specified and the results are requested in descending order, the end_time field is required. | [optional]
**order** | Option<**String**> | The order to list events in; either \"ASC\" or \"DESC\". Defaults to \"DESC\". | [optional][default to Desc]
**start_time** | Option<**i64**> | The start time in epoch milliseconds. If empty, returns events starting from the beginning of time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


