# SqlCreateAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the alert. | 
**options** | [**crate::models::SqlAlertOptions**](SqlAlertOptions.md) |  | 
**parent** | Option<**String**> | The identifier of the workspace folder containing the object. | [optional][default to folders/HOME]
**query_id** | [**String**](String.md) | Query ID. | 
**rearm** | Option<**i32**> | Number of seconds after being triggered before the alert rearms itself and can be triggered again. If `null`, alert will never be triggered again. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


