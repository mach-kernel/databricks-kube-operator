# PipelinesGetPipelineResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creator_user_name** | Option<**String**> | The username of the pipeline creator. | [optional]
**pipeline_id** | Option<**String**> | The ID of the pipeline. | [optional]
**run_as_user_name** | Option<**String**> | Username of the user that the pipeline will run on behalf of. | [optional]
**spec** | Option<[**crate::models::PipelinesPipelineSpec**](PipelinesPipelineSpec.md)> | The pipeline specification. This field is not returned when called by `ListPipelines`. | [optional]
**health** | Option<**String**> | The health of a pipeline. | [optional]
**last_modified** | Option<**i64**> | The last time the pipeline settings were modified or created. | [optional]
**name** | Option<**String**> | A human friendly identifier for the pipeline, taken from the `spec`. | [optional]
**cause** | Option<**String**> | An optional message detailing the cause of the pipeline state. | [optional]
**cluster_id** | Option<**String**> | The ID of the cluster that the pipeline is running on. | [optional]
**state** | Option<[**crate::models::PipelinesPipelineState**](PipelinesPipelineState.md)> |  | [optional]
**latest_updates** | Option<[**Vec<crate::models::PipelinesUpdateStateInfo>**](PipelinesUpdateStateInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


