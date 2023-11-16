# BillingLogDeliveryConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_id** | Option<[**String**](String.md)> | Databricks log delivery configuration ID. | [optional]
**storage_configuration_id** | Option<[**String**](String.md)> | The ID for a method:storage/create  that represents the S3 bucket with bucket policy as described in the main billable usage documentation page. See [Configure billable usage delivery](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html). | [optional]
**output_format** | Option<[**crate::models::BillingOutputFormat**](BillingOutputFormat.md)> |  | [optional]
**delivery_start_time** | Option<**String**> | This field applies only if `log_type` is `BILLABLE_USAGE`. This is the optional start month and year for delivery, specified in `YYYY-MM` format. Defaults to current year and month.  `BILLABLE_USAGE` logs are not available for usage before March 2019 (`2019-03`). | [optional]
**creation_time** | Option<**i64**> | Time in epoch milliseconds when the log delivery configuration was created. | [optional]
**workspace_ids_filter** | Option<**Vec<i64>**> |  | [optional]
**config_name** | Option<**String**> | The optional human-readable name of the log delivery configuration. Defaults to empty. | [optional]
**update_time** | Option<**i64**> | Time in epoch milliseconds when the log delivery configuration was updated. | [optional]
**status** | Option<[**crate::models::BillingLogDeliveryConfigStatus**](BillingLogDeliveryConfigStatus.md)> |  | [optional]
**credentials_id** | Option<[**String**](String.md)> | The ID for a method:credentials/create that represents the AWS IAM role with policy and trust relationship as described in the main billable usage documentation page. See [Configure billable usage delivery](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html). | [optional]
**log_delivery_status** | Option<[**crate::models::BillingLogDeliveryStatus**](BillingLogDeliveryStatus.md)> |  | [optional]
**delivery_path_prefix** | Option<**String**> | The optional delivery path prefix within Amazon S3 storage. Defaults to empty, which means that logs are delivered to the root of the bucket. This must be a valid S3 object key. This must not start or end with a slash character. | [optional]
**account_id** | Option<[**String**](String.md)> | The Databricks account ID that hosts the log delivery configuration. | [optional]
**log_type** | Option<[**crate::models::BillingLogType**](BillingLogType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


