# MlCreateRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_id** | Option<**String**> | ID of the associated experiment. | [optional]
**start_time** | Option<**i64**> | Unix timestamp in milliseconds of when the run started. | [optional]
**tags** | Option<[**Vec<crate::models::MlRunTag>**](MlRunTag.md)> |  | [optional]
**user_id** | Option<**String**> | ID of the user executing the run. This field is deprecated as of MLflow 1.0, and will be removed in a future MLflow release. Use 'Mlflowuser' tag instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


