# JobsGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | Option<**i64**> | The canonical identifier for this job. | [optional]
**creator_user_name** | Option<**String**> | The creator user name. This field won’t be included in the response if the user has been deleted. | [optional]
**run_as_user_name** | Option<**String**> | The user name that the job runs as. `run_as_user_name` is based on the current job settings, and is set to the creator of the job if job access control is disabled, or the `is_owner` permission if job access control is enabled. | [optional]
**settings** | Option<[**crate::models::JobSettings**](JobSettings.md)> |  | [optional]
**created_time** | Option<**i64**> | The time at which this job was created in epoch milliseconds (milliseconds since 1/1/1970 UTC). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


