# SharingPartitionValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the partition column. | [optional]
**op** | Option<**String**> | The operator to apply for the value. | [optional]
**recipient_property_key** | Option<**String**> | The key of a Delta Sharing recipient's property. For example `databricks-account-id`. When this field is set, field `value` can not be set. | [optional]
**value** | Option<**String**> | The value of the partition column. When this value is not set, it means `null` value. When this field is set, field `recipient_property_key` can not be set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


