# MlSearchExperiments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | Option<**String**> | String representing a SQL filter condition (Eg. \"name ILIKE 'my-experiment%'\") | [optional]
**max_results** | Option<**i64**> | Maximum number of experiments desired. Max threshold is 3000. | [optional]
**order_by** | Option<**Vec<String>**> |  | [optional]
**page_token** | Option<**String**> | Token indicating the page of experiments to fetch | [optional]
**view_type** | Option<**String**> | Qualifier for type of experiments to be returned. If unspecified, return only active experiments. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


