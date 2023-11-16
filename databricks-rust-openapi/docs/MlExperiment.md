# MlExperiment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifact_location** | Option<**String**> | Location where artifacts for the experiment are stored. | [optional]
**creation_time** | Option<**i64**> | Creation time | [optional]
**experiment_id** | Option<**String**> | Unique identifier for the experiment. | [optional]
**last_update_time** | Option<**i64**> | Last update time | [optional]
**lifecycle_stage** | Option<**String**> | Current life cycle stage of the experiment: \"active\" or \"deleted\". Deleted experiments are not returned by APIs. | [optional]
**name** | Option<**String**> | Human readable name that identifies the experiment. | [optional]
**tags** | Option<[**Vec<crate::models::MlExperimentTag>**](MlExperimentTag.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


