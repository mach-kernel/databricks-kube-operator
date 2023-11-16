# MlSetTag

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Name of the tag. Maximum size depends on storage backend. All storage backends are guaranteed to support key values up to 250 bytes in size. | 
**run_id** | Option<**String**> | ID of the run under which to log the tag. Must be provided. | [optional]
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run under which to log the tag. This field will be removed in a future MLflow version. | [optional]
**value** | **String** | String value of the tag being logged. Maximum size depends on storage backend. All storage backends are guaranteed to support key values up to 5000 bytes in size. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


