# ProvisioningCustomerManagedKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<[**String**](String.md)> | The Databricks account ID that holds the customer-managed key. | [optional]
**aws_key_info** | Option<[**crate::models::ProvisioningAwsKeyInfo**](ProvisioningAwsKeyInfo.md)> |  | [optional]
**creation_time** | Option<**i64**> | Time in epoch milliseconds when the customer key was created. | [optional]
**customer_managed_key_id** | Option<[**String**](String.md)> | ID of the encryption key configuration object. | [optional]
**gcp_key_info** | Option<[**crate::models::ProvisioningGcpKeyInfo**](ProvisioningGcpKeyInfo.md)> |  | [optional]
**use_cases** | Option<[**Vec<crate::models::ProvisioningKeyUseCase>**](ProvisioningKeyUseCase.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


