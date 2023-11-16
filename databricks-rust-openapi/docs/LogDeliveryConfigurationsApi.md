# \LogDeliveryConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**log_deliverycreate**](LogDeliveryConfigurationsApi.md#log_deliverycreate) | **POST** /api/2.0/accounts/{account_id}/log-delivery | Create a new log delivery configuration
[**log_deliveryget**](LogDeliveryConfigurationsApi.md#log_deliveryget) | **GET** /api/2.0/accounts/{account_id}/log-delivery/{log_delivery_configuration_id} | Get log delivery configuration
[**log_deliverylist**](LogDeliveryConfigurationsApi.md#log_deliverylist) | **GET** /api/2.0/accounts/{account_id}/log-delivery | Get all log delivery configurations
[**log_deliverypatch_status**](LogDeliveryConfigurationsApi.md#log_deliverypatch_status) | **PATCH** /api/2.0/accounts/{account_id}/log-delivery/{log_delivery_configuration_id} | Enable or disable log delivery configuration



## log_deliverycreate

> crate::models::BillingWrappedLogDeliveryConfiguration log_deliverycreate(account_id, billing_wrapped_create_log_delivery_configuration)
Create a new log delivery configuration

Creates a new Databricks log delivery configuration to enable delivery of the specified type of logs to your storage location. This requires that you already created a [credential object](#operation/create-credential-config) (which encapsulates a cross-account service IAM role) and a [storage configuration object](#operation/create-storage-config) (which encapsulates an S3 bucket).  For full details, including the required IAM role policies and bucket policies, see [Deliver and access billable usage logs](https://Docsdatabricks.com/administration-guide/account-settings/billable-usage-delivery.html) or [Configure audit logging](https://Docsdatabricks.com/administration-guide/account-settings/audit-logs.html).  **Note**: There is a limit on the number of log delivery configurations available per account (each limit applies separately to each log type including billable usage and audit logs). You can create a maximum of two enabled account-level delivery configurations (configurations without a workspace filter) per type. Additionally, you can create two enabled workspace-level delivery configurations per workspace for each log type, which means that the same workspace ID can occur in the workspace filter for no more than two delivery configurations per log type.  You cannot delete a log delivery configuration, but you can disable it (see [Enable or disable log delivery configuration](#operation/patch-log-delivery-config-status)).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**billing_wrapped_create_log_delivery_configuration** | [**BillingWrappedCreateLogDeliveryConfiguration**](BillingWrappedCreateLogDeliveryConfiguration.md) | Properties of the new log delivery configuration. | [required] |

### Return type

[**crate::models::BillingWrappedLogDeliveryConfiguration**](BillingWrappedLogDeliveryConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_deliveryget

> crate::models::BillingWrappedLogDeliveryConfiguration log_deliveryget(account_id, log_delivery_configuration_id)
Get log delivery configuration

Gets a Databricks log delivery configuration object for an account, both specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**log_delivery_configuration_id** | [**serde_json::Value**](.md) | Databricks log delivery configuration ID | [required] |

### Return type

[**crate::models::BillingWrappedLogDeliveryConfiguration**](BillingWrappedLogDeliveryConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_deliverylist

> crate::models::BillingWrappedLogDeliveryConfigurations log_deliverylist(account_id, status, credentials_id, storage_configuration_id)
Get all log delivery configurations

Gets all Databricks log delivery configurations associated with an account specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**status** | Option<[**BillingLogDeliveryConfigStatus**](.md)> | Filter by status `ENABLED` or `DISABLED`. |  |
**credentials_id** | Option<**String**> | Filter by credential configuration ID. |  |
**storage_configuration_id** | Option<**String**> | Filter by storage configuration ID. |  |

### Return type

[**crate::models::BillingWrappedLogDeliveryConfigurations**](BillingWrappedLogDeliveryConfigurations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_deliverypatch_status

> serde_json::Value log_deliverypatch_status(account_id, log_delivery_configuration_id, billing_update_log_delivery_configuration_status_request)
Enable or disable log delivery configuration

Enables or disables a log delivery configuration. Deletion of delivery configurations is not supported, so disable log delivery configurations that are no longer needed. Note that you can't re-enable a delivery configuration if this would violate the delivery configuration limits described under [Create log delivery](#operation/create-log-delivery-config).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**log_delivery_configuration_id** | [**serde_json::Value**](.md) | Databricks log delivery configuration ID | [required] |
**billing_update_log_delivery_configuration_status_request** | [**BillingUpdateLogDeliveryConfigurationStatusRequest**](BillingUpdateLogDeliveryConfigurationStatusRequest.md) | The new status for this log delivery configuration object. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

