# SqlAlertQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | The ID of the user who created this query. | [optional]
**is_archived** | Option<**bool**> | Indicates whether the query is trashed. Trashed queries can't be used in dashboards, or appear in search results. If this boolean is `true`, the `options` property for this query includes a `moved_to_trash_at` timestamp. Trashed queries are permanently deleted after 30 days. | [optional]
**query** | Option<**String**> | The text of the query to be run. | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**id** | Option<[**String**](String.md)> | Query ID. | [optional]
**name** | Option<**String**> | The title of this query that appears in list views, widget headings, and on the query page. | [optional]
**updated_at** | Option<**String**> | The timestamp at which this query was last updated. | [optional]
**is_safe** | Option<**bool**> | Text parameter types are not safe from SQL injection for all types of data source. Set this Boolean parameter to `true` if a query either does not use any text type parameters or uses a data source type where text type parameters are handled safely. | [optional]
**is_draft** | Option<**bool**> | Whether the query is a draft. Draft queries only appear in list views for their owners. Visualizations from draft queries cannot appear on dashboards. | [optional]
**data_source_id** | Option<[**String**](String.md)> | Data source ID. | [optional]
**created_at** | Option<**String**> | The timestamp when this query was created. | [optional]
**options** | Option<[**crate::models::SqlQueryOptions**](SqlQueryOptions.md)> |  | [optional]
**description** | Option<**String**> | General description that conveys additional information about this query such as usage notes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


