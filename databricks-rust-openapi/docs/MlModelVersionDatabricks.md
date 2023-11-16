# MlModelVersionDatabricks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The username of the user that created the object. | [optional]
**tags** | Option<[**Vec<crate::models::MlModelVersionTag>**](MlModelVersionTag.md)> |  | [optional]
**name** | Option<**String**> | Name of the model. | [optional]
**status** | Option<[**crate::models::Mlstatus**](Mlstatus.md)> |  | [optional]
**run_link** | Option<**String**> | URL of the run associated with the model artifacts. This field is set at model version creation time only for model versions whose source run is from a tracking server that is different from the registry server. | [optional]
**source** | Option<**String**> | URI that indicates the location of the source model artifacts. This is used when creating the model version. | [optional]
**run_id** | Option<[**String**](String.md)> | Unique identifier for the MLflow tracking run associated with the source model artifacts. | [optional]
**status_message** | Option<**String**> | Details on the current status, for example why registration failed. | [optional]
**version** | Option<**String**> | Version of the model. | [optional]
**last_updated_timestamp** | Option<**i64**> | Time of the object at last update, as a Unix timestamp in milliseconds. | [optional]
**creation_timestamp** | Option<**i64**> | Creation time of the object, as a Unix timestamp in milliseconds. | [optional]
**description** | Option<**String**> | User-specified description for the object. | [optional]
**permission_level** | Option<[**crate::models::MlPermissionLevel**](MlPermissionLevel.md)> |  | [optional]
**current_stage** | Option<[**crate::models::Mlstage**](Mlstage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


