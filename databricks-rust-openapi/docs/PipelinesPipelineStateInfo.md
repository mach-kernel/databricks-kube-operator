# PipelinesPipelineStateInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | Option<**String**> | The unique identifier of the cluster running the pipeline. | [optional]
**creator_user_name** | Option<**String**> | The username of the pipeline creator. | [optional]
**latest_updates** | Option<[**Vec<crate::models::PipelinesUpdateStateInfo>**](PipelinesUpdateStateInfo.md)> |  | [optional]
**name** | Option<**String**> | The user-friendly name of the pipeline. | [optional]
**pipeline_id** | Option<**String**> | The unique identifier of the pipeline. | [optional]
**run_as_user_name** | Option<**String**> | The username that the pipeline runs as. This is a read only value derived from the pipeline owner. | [optional]
**state** | Option<[**crate::models::PipelinesPipelineState**](PipelinesPipelineState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


