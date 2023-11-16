# PipelinesPipelineLibrary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file** | Option<[**crate::models::PipelinesFileLibrary**](PipelinesFileLibrary.md)> | The path to a file that defines a pipeline and is stored in the Databricks Repos.  | [optional]
**jar** | Option<**String**> | URI of the jar to be installed. Currently only DBFS is supported.  | [optional]
**maven** | Option<[**crate::models::ComputeMavenLibrary**](ComputeMavenLibrary.md)> | Specification of a maven library to be installed.  | [optional]
**notebook** | Option<[**crate::models::PipelinesNotebookLibrary**](PipelinesNotebookLibrary.md)> | The path to a notebook that defines a pipeline and is stored in the <Databricks> workspace.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


