# SqlQueryInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plans_state** | Option<[**crate::models::SqlPlansState**](SqlPlansState.md)> |  | [optional]
**endpoint_id** | Option<**String**> | Alias for `warehouse_id`. | [optional]
**user_id** | Option<**i32**> | The ID of the user who ran the query. | [optional]
**query_end_time_ms** | Option<**i32**> | The time the query ended. | [optional]
**rows_produced** | Option<**i32**> | The number of results returned by the query. | [optional]
**spark_ui_url** | Option<**String**> | URL to the query plan. | [optional]
**executed_as_user_id** | Option<**i32**> | The ID of the user whose credentials were used to run the query. | [optional]
**channel_used** | Option<[**crate::models::SqlChannelInfo**](SqlChannelInfo.md)> |  | [optional]
**user_name** | Option<**String**> | The email address or username of the user who ran the query. | [optional]
**warehouse_id** | Option<**String**> | Warehouse ID. | [optional]
**lookup_key** | Option<**String**> | A key that can be used to look up query details. | [optional]
**execution_end_time_ms** | Option<**i32**> | The time execution of the query ended. | [optional]
**can_subscribe_to_live_query** | Option<**bool**> | Reserved for internal use. | [optional]
**is_final** | Option<**bool**> | Whether more updates for the query are expected. | [optional]
**query_start_time_ms** | Option<**i32**> | The time the query started. | [optional]
**status** | Option<[**crate::models::SqlQueryStatus**](SqlQueryStatus.md)> |  | [optional]
**error_message** | Option<**String**> | Message describing why the query could not complete. | [optional]
**query_id** | Option<**String**> | The query ID. | [optional]
**statement_type** | Option<[**crate::models::SqlQueryStatementType**](SqlQueryStatementType.md)> |  | [optional]
**duration** | Option<**i32**> | Total execution time of the query from the client’s point of view, in milliseconds. | [optional]
**executed_as_user_name** | Option<**String**> | The email address or username of the user whose credentials were used to run the query. | [optional]
**query_text** | Option<**String**> | The text of the query. | [optional]
**metrics** | Option<[**crate::models::SqlQueryMetrics**](SqlQueryMetrics.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


