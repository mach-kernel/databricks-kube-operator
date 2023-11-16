# MlCreateTransitionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | User-provided comment on the action. | [optional]
**name** | **String** | Name of the model. | 
**stage** | [**crate::models::Mlstage**](Mlstage.md) | Target stage of the transition. Valid values are:  * `None`: The initial stage of a model version.  * `Staging`: Staging or pre-production stage.  * `Production`: Production stage.  * `Archived`: Archived stage. | 
**version** | **String** | Version of the model. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


