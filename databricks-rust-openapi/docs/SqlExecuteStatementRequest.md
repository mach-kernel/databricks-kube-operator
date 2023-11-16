# SqlExecuteStatementRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**warehouse_id** | Option<**String**> | Warehouse upon which to execute a statement. See also [What are SQL warehouses?](/sql/admin/warehouse-Typehtml)  | [optional]
**schema** | Option<**String**> | Sets default schema for statement execution, similar to [`USE SCHEMA`](https://Docsdatabricks.com/sql/language-manual/sql-ref-syntax-ddl-use-schema.html) in SQL.  | [optional]
**disposition** | Option<[**crate::models::SqlDisposition**](SqlDisposition.md)> |  | [optional]
**wait_timeout** | Option<**String**> | The time in seconds the API service will wait for the statement's result set as `Ns`, where `N` can be set to 0 or to a value between 5 and 50. When set to '0s' the statement will execute in asynchronous mode.\"  | [optional]
**statement** | Option<**String**> | SQL statement to execute | [optional]
**byte_limit** | Option<**i64**> | Applies the given byte limit to the statement's result size. Byte counts are based on internal representations and may not match measurable sizes in the requested `format`.  | [optional]
**catalog** | Option<**String**> | Sets default catalog for statement execution, similar to [`USE CATALOG`](https://Docsdatabricks.com/sql/language-manual/sql-ref-syntax-ddl-use-catalog.html) in SQL.  | [optional]
**format** | Option<[**crate::models::SqlFormat**](SqlFormat.md)> |  | [optional]
**on_wait_timeout** | Option<[**crate::models::SqlTimeoutAction**](SqlTimeoutAction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


