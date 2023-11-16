# MlTransitionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_actions** | Option<[**Vec<crate::models::MlActivityAction>**](MlActivityAction.md)> |  | [optional]
**comment** | Option<**String**> | User-provided comment associated with the transition request. | [optional]
**creation_timestamp** | Option<**i64**> | Creation time of the object, as a Unix timestamp in milliseconds. | [optional]
**to_stage** | Option<[**crate::models::Mlstage**](Mlstage.md)> | Target stage of the transition. Valid values are:  * `None`: The initial stage of a model version.  * `Staging`: Staging or pre-production stage.  * `Production`: Production stage.  * `Archived`: Archived stage. | [optional]
**user_id** | Option<**String**> | The username of the user that created the object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


