# PipelinesPipelineSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**development** | Option<**bool**> | Whether the pipeline is in Development mode. Defaults to false. | [optional]
**libraries** | Option<[**Vec<crate::models::PipelinesPipelineLibrary>**](PipelinesPipelineLibrary.md)> |  | [optional]
**photon** | Option<**bool**> | Whether Photon is enabled for this pipeline. | [optional]
**storage** | Option<**String**> | DBFS root directory for storing checkpoints and tables. | [optional]
**clusters** | Option<[**Vec<crate::models::PipelinesPipelineCluster>**](PipelinesPipelineCluster.md)> |  | [optional]
**id** | Option<**String**> | Unique identifier for this pipeline. | [optional]
**configuration** | Option<**::std::collections::HashMap<String, String>**> | String-String configuration for this pipeline execution. | [optional]
**filters** | Option<[**crate::models::PipelinesFilters**](PipelinesFilters.md)> | Filters on which Pipeline packages to include in the deployed graph. | [optional]
**name** | Option<**String**> | Friendly identifier for this pipeline. | [optional]
**edition** | Option<**String**> | Pipeline product edition. | [optional]
**target** | Option<**String**> | Target schema (database) to add tables in this pipeline to. If not specified, no data is published to the Hive metastore or Unity Catalog. To publish to Unity Catalog, also specify `catalog`. | [optional]
**channel** | Option<**String**> | DLT Release Channel that specifies which version to use. | [optional]
**continuous** | Option<**bool**> | Whether the pipeline is continuous or triggered. This replaces `trigger`. | [optional]
**trigger** | Option<[**crate::models::PipelinesPipelineTrigger**](PipelinesPipelineTrigger.md)> | Which pipeline trigger to use. Deprecated: Use `continuous` instead. | [optional]
**catalog** | Option<**String**> | A catalog in Unity Catalog to publish data from this pipeline to. If `target` is specified, tables in this pipeline are published to a `target` schema inside `catalog` (for example, `catalog`.`target`.`table`). If `target` is not specified, no data is published to Unity Catalog. | [optional]
**serverless** | Option<**bool**> | Whether serverless compute is enabled for this pipeline. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


