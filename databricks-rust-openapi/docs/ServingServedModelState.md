# ServingServedModelState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deployment** | Option<**String**> | The state of the served model deployment. DEPLOYMENT_CREATING indicates that the served model is not ready yet because the deployment is still being created (Ie container image is building, model server is deploying for the first time, etc.). DEPLOYMENT_RECOVERING indicates that the served model was previously in a ready state but no longer is and is attempting to recover. DEPLOYMENT_READY indicates that the served model is ready to receive traffic. DEPLOYMENT_FAILED indicates that there was an error trying to bring up the served model (e.g container image build failed, the model server failed to start due to a model loading error, etc.) DEPLOYMENT_ABORTED indicates that the deployment was terminated likely due to a failure in bringing up another  served model under the same endpoint and config version.  | [optional]
**deployment_state_message** | Option<**String**> | More information about the state of the served model, if available. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


