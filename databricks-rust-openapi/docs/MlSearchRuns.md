# MlSearchRuns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_ids** | Option<**Vec<String>**> |  | [optional]
**filter** | Option<**String**> | A filter expression over params, metrics, and tags, that allows returning a subset of runs. The syntax is a subset of SQL that supports ANDing together binary operations between a param, metric, or tag and a constant.  Example: `Metricsrmse < 1 and params.model_class = 'LogisticRegression'`  You can select columns with special characters (hyphen, space, period, etc.) by using double quotes: `metrics.\"model class\" = 'LinearRegression' and tags.\"user-name\" = 'Tomas'`  Supported operators are `=`, `!=`, `>`, `>=`, `<`, and `<=`. | [optional]
**max_results** | Option<**i32**> | Maximum number of runs desired. Max threshold is 50000 | [optional][default to 1000]
**order_by** | Option<**Vec<String>**> |  | [optional]
**page_token** | Option<**String**> | Token for the current page of runs. | [optional]
**run_view_type** | Option<**String**> | Whether to display only active, only deleted, or all runs. Defaults to only active runs. | [optional][default to ActiveOnly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


