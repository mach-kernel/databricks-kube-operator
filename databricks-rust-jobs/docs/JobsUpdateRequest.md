# JobsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **i64** | The canonical identifier of the job to update. This field is required. | 
**new_settings** | Option<[**crate::models::JobSettings**](JobSettings.md)> |  | [optional]
**fields_to_remove** | Option<**Vec<String>**> | Remove top-level fields in the job settings. Removing nested fields is not supported. This field is optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


