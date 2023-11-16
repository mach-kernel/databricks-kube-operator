# ServingServingEndpointDetailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**crate::models::ServingEndpointCoreConfigOutput**](ServingEndpointCoreConfigOutput.md)> | The config that is currently being served by the endpoint. | [optional]
**pending_config** | Option<[**crate::models::ServingEndpointPendingConfig**](ServingEndpointPendingConfig.md)> | The config that the endpoint is attempting to update to. | [optional]
**id** | Option<**String**> | System-generated ID of the endpoint. This is used to refer to the endpoint in the Permissions API | [optional]
**creator** | Option<**String**> | The email of the user who created the serving endpoint. | [optional]
**name** | Option<**String**> | The name of the serving endpoint. | [optional]
**state** | Option<[**crate::models::ServingEndpointState**](ServingEndpointState.md)> | Information corresponding to the state of the serving endpoint. | [optional]
**last_updated_timestamp** | Option<**i64**> | The timestamp when the endpoint was last updated by a user in Unix time. | [optional]
**creation_timestamp** | Option<**i64**> | The timestamp when the endpoint was created in Unix time. | [optional]
**permission_level** | Option<**String**> | The permission level of the principal making the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


