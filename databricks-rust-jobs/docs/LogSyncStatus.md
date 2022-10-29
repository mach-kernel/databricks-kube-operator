# LogSyncStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_attempted** | Option<**i64**> | The timestamp of last attempt. If the last attempt fails, last_exception contains the exception in the last attempt. | [optional]
**last_exception** | Option<**String**> | The exception thrown in the last attempt, it would be null (omitted in the response) if there is no exception in last attempted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


