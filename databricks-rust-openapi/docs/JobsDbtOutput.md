# JobsDbtOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifacts_headers** | Option<**::std::collections::HashMap<String, String>**> | An optional map of headers to send when retrieving the artifact from the `artifacts_link`. | [optional]
**artifacts_link** | Option<**String**> | A pre-signed URL to download the (compressed) dbt artifacts. This link is valid for a limited time (30 minutes). This information is only available after the run has finished. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


