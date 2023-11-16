# MlActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The username of the user that created the object. | [optional]
**activity_type** | Option<[**crate::models::MlActivityType**](MlActivityType.md)> |  | [optional]
**id** | Option<[**String**](String.md)> | Unique identifier for the object. | [optional]
**to_stage** | Option<[**crate::models::Mlstage**](Mlstage.md)> | Target stage of the transition. Valid values are:  * `None`: The initial stage of a model version.  * `Staging`: Staging or pre-production stage.  * `Production`: Production stage.  * `Archived`: Archived stage. | [optional]
**from_stage** | Option<[**crate::models::Mlstage**](Mlstage.md)> | Source stage of the transition (if the activity is stage transition related). Valid values are:  * `None`: The initial stage of a model version.  * `Staging`: Staging or pre-production stage.  * `Production`: Production stage.  * `Archived`: Archived stage. | [optional]
**system_comment** | Option<**String**> | Comment made by system, for example explaining an activity of type `SYSTEM_TRANSITION`. It usually describes a side effect, such as a version being archived as part of another version's stage transition, and may not be returned for some activity types. | [optional]
**comment** | Option<**String**> | User-provided comment associated with the activity. | [optional]
**last_updated_timestamp** | Option<**i64**> | Time of the object at last update, as a Unix timestamp in milliseconds. | [optional]
**creation_timestamp** | Option<**i64**> | Creation time of the object, as a Unix timestamp in milliseconds. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


