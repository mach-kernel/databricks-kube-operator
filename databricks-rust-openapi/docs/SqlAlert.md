# SqlAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent** | Option<**String**> | The identifier of the workspace folder containing the object. | [optional][default to folders/HOME]
**query** | Option<[**crate::models::SqlAlertQuery**](SqlAlertQuery.md)> |  | [optional]
**user** | Option<[**crate::models::SqlUser**](SqlUser.md)> |  | [optional]
**id** | Option<[**String**](String.md)> | Alert ID. | [optional]
**name** | Option<**String**> | Name of the alert. | [optional]
**rearm** | Option<**i32**> | Number of seconds after being triggered before the alert rearms itself and can be triggered again. If `null`, alert will never be triggered again. | [optional]
**updated_at** | Option<**String**> | Timestamp when the alert was last updated. | [optional]
**last_triggered_at** | Option<**String**> | Timestamp when the alert was last triggered. | [optional]
**state** | Option<[**crate::models::SqlalertState**](Sqlalert_state.md)> |  | [optional]
**created_at** | Option<**String**> | Timestamp when the alert was created. | [optional]
**options** | Option<[**crate::models::SqlAlertOptions**](SqlAlertOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


