# ComputeAzureAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**crate::models::ComputeAzureAvailability**](ComputeAzureAvailability.md)> |  | [optional]
**first_on_demand** | Option<**i32**> | The first `first_on_demand` nodes of the cluster will be placed on on-demand instances. This value should be greater than 0, to make sure the cluster driver node is placed on an on-demand instance. If this value is greater than or equal to the current cluster size, all nodes will be placed on on-demand instances. If this value is less than the current cluster size, `first_on_demand` nodes will be placed on on-demand instances and the remainder will be placed on `availability` instances. Note that this value does not affect cluster size and cannot currently be mutated over the lifetime of a cluster. | [optional][default to 1]
**log_analytics_info** | Option<[**crate::models::ComputeLogAnalyticsInfo**](ComputeLogAnalyticsInfo.md)> | Defines values necessary to configure and run Azure Log Analytics agent | [optional]
**spot_bid_max_price** | Option<**f64**> | The max bid price to be used for Azure spot instances. The Max price for the bid cannot be higher than the on-demand price of the instance. If not specified, the default value is -1, which specifies that the instance cannot be evicted on the basis of price, and only on the basis of availability. Further, the value should > 0 or -1. | [optional][default to -1.0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


