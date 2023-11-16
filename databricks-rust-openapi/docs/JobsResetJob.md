# JobsResetJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **i64** | The canonical identifier of the job to reset. This field is required. | 
**new_settings** | [**crate::models::JobsJobSettings**](JobsJobSettings.md) | The new settings of the job. These settings completely replace the old settings.  Changes to the field `JobBaseSettingstimeout_seconds` are applied to active runs. Changes to other fields are applied to future runs only.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


