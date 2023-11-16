# ServingEndpointState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_update** | Option<**String**> | The state of an endpoint's config update. This informs the user if the pending_config  is in progress, if the update failed, or if there is no update in progress. Note that if the endpoint's config_update state value is IN_PROGRESS, another update can not be made until the update completes or fails.\"  | [optional]
**ready** | Option<**String**> | The state of an endpoint, indicating whether or not the endpoint is queryable. An endpoint is READY if all of the served models in its active configuration are ready. If any of the actively served models are in a non-ready state, the endpoint state will be NOT_READY.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


