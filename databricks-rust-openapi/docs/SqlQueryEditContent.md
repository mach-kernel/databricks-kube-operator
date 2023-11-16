# SqlQueryEditContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data_source_id** | Option<[**String**](String.md)> | Data source ID. | [optional]
**description** | Option<**String**> | General description that conveys additional information about this query such as usage notes. | [optional]
**name** | Option<**String**> | The title of this query that appears in list views, widget headings, and on the query page. | [optional]
**options** | Option<[**serde_json::Value**](.md)> | Exclusively used for storing a list parameter definitions. A parameter is an object with `title`, `name`, `type`, and `value` properties. The `value` field here is the default value. It can be overridden at runtime. | [optional]
**query** | Option<**String**> | The text of the query to be run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


