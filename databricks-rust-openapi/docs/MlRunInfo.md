# MlRunInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | User who initiated the run. This field is deprecated as of MLflow 1.0, and will be removed in a future MLflow release. Use 'Mlflowuser' tag instead. | [optional]
**artifact_uri** | Option<**String**> | URI of the directory where artifacts should be uploaded. This can be a local path (starting with \"/\"), or a distributed file system (DFS) path, like `s3://bucket/directory` or `dbfs:/my/directory`. If not set, the local `./mlruns` directory is  chosen. | [optional]
**experiment_id** | Option<**String**> | The experiment ID. | [optional]
**lifecycle_stage** | Option<**String**> | Current life cycle stage of the experiment : OneOf(\"active\", \"deleted\") | [optional]
**status** | Option<**String**> | Current status of the run. | [optional]
**end_time** | Option<**i64**> | Unix timestamp of when the run ended in milliseconds. | [optional]
**run_id** | Option<**String**> | Unique identifier for the run. | [optional]
**start_time** | Option<**i64**> | Unix timestamp of when the run started in milliseconds. | [optional]
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] Unique identifier for the run. This field will be removed in a future MLflow version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


