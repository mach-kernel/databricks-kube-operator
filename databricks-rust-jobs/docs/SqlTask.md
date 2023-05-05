# SqlTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<[**crate::models::SqlTaskQuery**](SqlTaskQuery.md)> |  | [optional]
**dashboard** | Option<[**crate::models::SqlTaskDashboard**](SqlTaskDashboard.md)> |  | [optional]
**alert** | Option<[**crate::models::SqlTaskAlert**](SqlTaskAlert.md)> |  | [optional]
**parameters** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Parameters to be used for each run of this job. The SQL alert task does not support custom parameters. | [optional]
**warehouse_id** | **String** | The canonical identifier of the SQL warehouse. Only serverless and pro SQL warehouses are supported. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


