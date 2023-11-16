# JobsUpdateJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fields_to_remove** | Option<**Vec<String>**> |  | [optional]
**job_id** | **i64** | The canonical identifier of the job to update. This field is required. | 
**new_settings** | Option<[**crate::models::JobsJobSettings**](JobsJobSettings.md)> | The new settings for the job.  Top-level fields specified in `new_settings` are completely replaced, except for arrays which are merged. That is, new and existing entries are completely replaced based on the respective key fields, Ie. `task_key` or `job_cluster_key`, while previous entries are kept.  Partially updating nested fields is not supported.  Changes to the field `JobSettings.timeout_seconds` are applied to active runs. Changes to other fields are applied to future runs only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


