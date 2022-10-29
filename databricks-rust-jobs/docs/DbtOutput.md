# DbtOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifacts_link** | Option<**String**> | A pre-signed URL to download the (compressed) dbt artifacts. This link is valid for a limited time (30 minutes). This information is only available after the run has finished. | [optional]
**artifacts_headers** | Option<[**serde_json::Value**](.md)> | An optional map of headers to send when retrieving the artifact from the `artifacts_link`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


