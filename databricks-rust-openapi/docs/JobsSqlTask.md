# JobsSqlTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert** | Option<[**crate::models::JobsSqlTaskAlert**](JobsSqlTaskAlert.md)> | If alert, indicates that this job must refresh a SQL alert. | [optional]
**dashboard** | Option<[**crate::models::JobsSqlTaskDashboard**](JobsSqlTaskDashboard.md)> | If dashboard, indicates that this job must refresh a SQL dashboard. | [optional]
**file** | Option<[**crate::models::JobsSqlTaskFile**](JobsSqlTaskFile.md)> | If file, indicates that this job runs a SQL file in a remote Git repository. Only one SQL statement is supported in a file. Multiple SQL statements separated by semicolons (;) are not permitted. | [optional]
**parameters** | Option<**::std::collections::HashMap<String, String>**> | Parameters to be used for each run of this job. The SQL alert task does not support custom parameters. | [optional]
**query** | Option<[**crate::models::JobsSqlTaskQuery**](JobsSqlTaskQuery.md)> | If query, indicates that this job must execute a SQL query. | [optional]
**warehouse_id** | **String** | The canonical identifier of the SQL warehouse. Only serverless and pro SQL warehouses are supported. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


