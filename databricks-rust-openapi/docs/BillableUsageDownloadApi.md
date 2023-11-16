# \BillableUsageDownloadApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**billable_usagedownload**](BillableUsageDownloadApi.md#billable_usagedownload) | **GET** /api/2.0/accounts/{account_id}/usage/download | Return billable usage logs



## billable_usagedownload

> String billable_usagedownload(start_month, end_month, account_id, personal_data)
Return billable usage logs

Returns billable usage logs in CSV format for the specified account and date range. For the data schema, see [CSV file schema](https://Docsdatabricks.com/administration-guide/account-settings/usage-analysis.html#schema). Note that this method might take multiple minutes to complete.  **Warning**: Depending on the queried date range, the number of workspaces in the account, the size of the response and the internet speed of the caller, this API may hit a timeout after a few minutes. If you experience this, try to mitigate by calling the API with narrower date ranges. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_month** | **String** | Format: `YYYY-MM`. First month to return billable usage logs for. This field is required. | [required] |
**end_month** | **String** | Format: `YYYY-MM`. Last month to return billable usage logs for. This field is required. | [required] |
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**personal_data** | Option<**bool**> | Specify whether to include personally identifiable information in the billable usage logs, for example the email addresses of cluster creators. Handle this information with care. Defaults to false. |  |[default to false]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

