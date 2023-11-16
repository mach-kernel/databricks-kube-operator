# SqlWidgetOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Timestamp when this object was created | [optional]
**dashboard_id** | Option<[**String**](String.md)> | The dashboard ID to which this widget belongs. Each widget can belong to one dashboard. | [optional]
**is_hidden** | Option<**bool**> | Whether this widget is hidden on the dashboard. | [optional][default to false]
**parameter_mappings** | Option<[**serde_json::Value**](.md)> | How parameters used by the visualization in this widget relate to other widgets on the dashboard. Databricks does not recommend modifying this definition in JSON. | [optional]
**position** | Option<[**serde_json::Value**](.md)> | Coordinates of this widget on a dashboard. This portion of the API changes frequently and is unsupported. | [optional]
**text** | Option<**String**> | If this is a textbox widget, the application displays this text. This field is ignored if the widget contains a visualization in the `visualization` field. | [optional]
**updated_at** | Option<**String**> | Timestamp of the last time this object was updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


