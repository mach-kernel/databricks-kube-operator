# JobsBaseJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_time** | Option<**i64**> | The time at which this job was created in epoch milliseconds (milliseconds since 1/1/1970 UTC). | [optional]
**creator_user_name** | Option<**String**> | The creator user name. This field won’t be included in the response if the user has already been deleted. | [optional]
**job_id** | Option<**i64**> | The canonical identifier for this job. | [optional]
**settings** | Option<[**crate::models::JobsJobSettings**](JobsJobSettings.md)> | Settings for this job and all of its runs. These settings can be updated using the `resetJob` method. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


