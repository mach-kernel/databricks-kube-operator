# MlSetModelVersionTagRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Name of the tag. Maximum size depends on storage backend. If a tag with this name already exists, its preexisting value will be replaced by the specified `value`. All storage backends are guaranteed to support key values up to 250 bytes in size. | 
**name** | **String** | Unique name of the model. | 
**value** | **String** | String value of the tag being logged. Maximum size depends on storage backend. All storage backends are guaranteed to support key values up to 5000 bytes in size. | 
**version** | **String** | Model version number. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


