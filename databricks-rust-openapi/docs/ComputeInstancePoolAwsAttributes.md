# ComputeInstancePoolAwsAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<**String**> | Availability type used for the spot nodes.  The default value is defined by InstancePoolConfinstancePoolDefaultAwsAvailability | [optional]
**spot_bid_price_percent** | Option<**i32**> | Calculates the bid price for AWS spot instances, as a percentage of the corresponding instance type's on-demand price. For example, if this field is set to 50, and the cluster needs a new `r3.xlarge` spot instance, then the bid price is half of the price of on-demand `r3.xlarge` instances. Similarly, if this field is set to 200, the bid price is twice the price of on-demand `r3.xlarge` instances. If not specified, the default value is 100. When spot instances are requested for this cluster, only spot instances whose bid price percentage matches this field will be considered. Note that, for safety, we enforce this field to be no more than 10000.  The default value and documentation here should be kept consistent with CommonConfdefaultSpotBidPricePercent and CommonConf.maxSpotBidPricePercent. | [optional][default to 100]
**zone_id** | Option<**String**> | Identifier for the availability zone/datacenter in which the cluster resides. This string will be of a form like \"us-west-2a\". The provided availability zone must be in the same region as the Databricks deployment. For example, \"us-west-2a\" is not a valid zone id if the Databricks deployment resides in the \"us-east-1\" region. This is an optional field at cluster creation, and if not specified, a default zone will be used. The list of available zones as well as the default value can be found by using the `List Zones`_ method. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


