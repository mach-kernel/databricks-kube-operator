# BillingLogDeliveryStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_attempt_time** | Option<**String**> | The UTC time for the latest log delivery attempt. | [optional]
**last_successful_attempt_time** | Option<**String**> | The UTC time for the latest successful log delivery. | [optional]
**message** | Option<**String**> | Informative message about the latest log delivery attempt. If the log delivery fails with USER_FAILURE, error details will be provided for fixing misconfigurations in cloud permissions. | [optional]
**status** | Option<[**crate::models::BillingDeliveryStatus**](BillingDeliveryStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


