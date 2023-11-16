# SqlAlertOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**column** | **String** | Name of column in the query result to compare in alert evaluation. | 
**custom_body** | Option<**String**> | Custom body of alert notification, if it exists. See [here](https://Docsdatabricks.com/sql/user/alerts/index.html) for custom templating instructions. | [optional]
**custom_subject** | Option<**String**> | Custom subject of alert notification, if it exists. This includes email subject, Slack notification header, etc. See [here](https://Docsdatabricks.com/sql/user/alerts/index.html) for custom templating instructions. | [optional]
**muted** | Option<**bool**> | Whether or not the alert is muted. If an alert is muted, it will not notify users and notification destinations when triggered. | [optional][default to false]
**op** | **String** | Operator used to compare in alert evaluation: `>`, `>=`, `<`, `<=`, `==`, `!=` | 
**value** | [**serde_json::Value**](.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


