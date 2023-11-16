# SqlDashboard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | The ID of the user that created and owns this dashboard. | [optional]
**dashboard_filters_enabled** | Option<**bool**> | In the web application, query filters that share a name are coupled to a single selection box if this value is `true`. | [optional][default to false]
**is_favorite** | Option<**bool**> | Indicates whether this query object appears in the current user's favorites list. This flag determines whether the star icon for favorites is selected. | [optional]
**is_archived** | Option<**bool**> | Indicates whether a dashboard is trashed. Trashed dashboards won't appear in list views.  If this boolean is `true`, the `options` property for this dashboard includes a `moved_to_trash_at` timestamp. Items in trash are permanently deleted after 30 days. | [optional]
**parent** | Option<**String**> | The identifier of the workspace folder containing the object. | [optional][default to folders/HOME]
**user** | Option<[**crate::models::SqlUser**](SqlUser.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**id** | Option<[**String**](String.md)> | The ID for this dashboard. | [optional]
**name** | Option<**String**> | The title of the dashboard that appears in list views and at the top of the dashboard page. | [optional]
**slug** | Option<**String**> | URL slug. Usually mirrors the query name with dashes (`-`) instead of spaces. Appears in the URL for this query. | [optional]
**widgets** | Option<[**Vec<crate::models::SqlWidget>**](SqlWidget.md)> |  | [optional]
**updated_at** | Option<**String**> | Timestamp when this dashboard was last updated. | [optional]
**permission_tier** | Option<[**crate::models::SqlPermissionLevel**](SqlPermissionLevel.md)> |  | [optional]
**is_draft** | Option<**bool**> | Whether a dashboard is a draft. Draft dashboards only appear in list views for their owners. | [optional]
**created_at** | Option<**String**> | Timestamp when this dashboard was created. | [optional]
**can_edit** | Option<**bool**> | Whether the authenticated user can edit the query definition. | [optional]
**options** | Option<[**crate::models::SqlDashboardOptions**](SqlDashboardOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


