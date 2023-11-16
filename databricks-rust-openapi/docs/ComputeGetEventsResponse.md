# ComputeGetEventsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | Option<[**Vec<crate::models::ComputeClusterEvent>**](ComputeClusterEvent.md)> |  | [optional]
**next_page** | Option<[**crate::models::ComputeGetEvents**](ComputeGetEvents.md)> | The parameters required to retrieve the next page of events. Omitted if there are no more events to read. | [optional]
**total_count** | Option<**i64**> | The total number of events filtered by the start_time, end_time, and event_types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


