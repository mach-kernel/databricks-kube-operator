# WorkspaceImport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | Option<**String**> | The base64-encoded content. This has a limit of 10 MB.  If the limit (10MB) is exceeded, exception with error code **MAX_NOTEBOOK_SIZE_EXCEEDED** is thrown. This parameter might be absent, and instead a posted file is used.  | [optional]
**format** | Option<[**crate::models::WorkspaceImportFormat**](WorkspaceImportFormat.md)> |  | [optional]
**language** | Option<[**crate::models::WorkspaceLanguage**](WorkspaceLanguage.md)> |  | [optional]
**overwrite** | Option<**bool**> | The flag that specifies whether to overwrite existing object. It is `false` by default. For `DBC` format, `overwrite` is not supported since it may contain a directory. | [optional][default to false]
**path** | **String** | The absolute path of the object or directory. Importing a directory is only supported for the `DBC` format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


