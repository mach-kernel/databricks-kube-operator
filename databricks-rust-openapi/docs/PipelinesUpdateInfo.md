# PipelinesUpdateInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pipeline_id** | Option<**String**> | The ID of the pipeline. | [optional]
**config** | Option<[**crate::models::PipelinesPipelineSpec**](PipelinesPipelineSpec.md)> | The pipeline configuration with system defaults applied where unspecified by the user. Not returned by ListUpdates. | [optional]
**refresh_selection** | Option<**Vec<String>**> |  | [optional]
**creation_time** | Option<**i64**> | The time when this update was created. | [optional]
**cause** | Option<**String**> | What triggered this update. | [optional]
**full_refresh_selection** | Option<**Vec<String>**> |  | [optional]
**full_refresh** | Option<**bool**> | If true, this update will reset all tables before running. | [optional]
**cluster_id** | Option<**String**> | The ID of the cluster that the update is running on. | [optional]
**state** | Option<**String**> | The update state. | [optional]
**update_id** | Option<**String**> | The ID of this update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


