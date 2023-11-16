# WorkspaceDelete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | **String** | The absolute path of the notebook or directory. | 
**recursive** | Option<**bool**> | The flag that specifies whether to delete the object recursively. It is `false` by default. Please note this deleting directory is not atomic. If it fails in the middle, some of objects under this directory may be deleted and cannot be undone. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


