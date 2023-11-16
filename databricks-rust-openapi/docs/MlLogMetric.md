# MlLogMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Name of the metric. | 
**run_id** | Option<**String**> | ID of the run under which to log the metric. Must be provided. | [optional]
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run under which to log the metric. This field will be removed in a future MLflow version. | [optional]
**step** | Option<**i64**> | Step at which to log the metric | [optional][default to 0]
**timestamp** | **i64** | Unix timestamp in milliseconds at the time metric was logged. | 
**value** | **f64** | Double value of the metric being logged. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


