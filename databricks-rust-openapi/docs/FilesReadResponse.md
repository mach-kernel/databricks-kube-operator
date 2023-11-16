# FilesReadResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bytes_read** | Option<**i64**> | The number of bytes read (could be less than `length` if we hit end of file). This refers to number of bytes read in unencoded version (response data is base64-encoded).  | [optional]
**data** | Option<**String**> | The base64-encoded contents of the file read. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


