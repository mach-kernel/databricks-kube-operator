# BillingCreateLogDeliveryConfigurationParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**storage_configuration_id** | [**String**](String.md) | The ID for a method:storage/create  that represents the S3 bucket with bucket policy as described in the main billable usage documentation page. See [Configure billable usage delivery](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html). | 
**output_format** | [**crate::models::BillingOutputFormat**](BillingOutputFormat.md) |  | 
**delivery_start_time** | Option<**String**> | This field applies only if `log_type` is `BILLABLE_USAGE`. This is the optional start month and year for delivery, specified in `YYYY-MM` format. Defaults to current year and month.  `BILLABLE_USAGE` logs are not available for usage before March 2019 (`2019-03`). | [optional]
**workspace_ids_filter** | Option<**Vec<i64>**> |  | [optional]
**config_name** | Option<**String**> | The optional human-readable name of the log delivery configuration. Defaults to empty. | [optional]
**status** | Option<[**crate::models::BillingLogDeliveryConfigStatus**](BillingLogDeliveryConfigStatus.md)> |  | [optional]
**credentials_id** | [**String**](String.md) | The ID for a method:credentials/create that represents the AWS IAM role with policy and trust relationship as described in the main billable usage documentation page. See [Configure billable usage delivery](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html). | 
**delivery_path_prefix** | Option<**String**> | The optional delivery path prefix within Amazon S3 storage. Defaults to empty, which means that logs are delivered to the root of the bucket. This must be a valid S3 object key. This must not start or end with a slash character. | [optional]
**log_type** | [**crate::models::BillingLogType**](BillingLogType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


