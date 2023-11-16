# JobsJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_time** | Option<**i64**> | The time at which this job was created in epoch milliseconds (milliseconds since 1/1/1970 UTC). | [optional]
**creator_user_name** | Option<**String**> | The creator user name. This field won’t be included in the response if the user has already been deleted. | [optional]
**job_id** | Option<**i64**> | The canonical identifier for this job. | [optional]
**run_as_user_name** | Option<**String**> | The email of an active workspace user or the application ID of a service principal that the job runs as. This value can be changed by setting the `run_as` field when creating or updating a job.  By default, `run_as_user_name` is based on the current job settings and is set to the creator of the job if job access control is disabled or to the user with the `is_owner` permission if job access control is enabled.  | [optional]
**settings** | Option<[**crate::models::JobsJobSettings**](JobsJobSettings.md)> | Settings for this job and all of its runs. These settings can be updated using the `resetJob` method. | [optional]
**trigger_history** | Option<[**crate::models::JobsTriggerHistory**](JobsTriggerHistory.md)> | History of the file arrival trigger associated with the job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


