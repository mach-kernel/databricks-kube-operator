# JobsSqlAlertOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_state** | Option<[**crate::models::JobsSqlAlertState**](JobsSqlAlertState.md)> |  | [optional]
**output_link** | Option<**String**> | The link to find the output results. | [optional]
**query_text** | Option<**String**> | The text of the SQL query. Can Run permission of the SQL query associated with the SQL alert is required to view this field. | [optional]
**sql_statements** | Option<[**Vec<crate::models::JobsSqlStatementOutput>**](JobsSqlStatementOutput.md)> |  | [optional]
**warehouse_id** | Option<**String**> | The canonical identifier of the SQL warehouse. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


