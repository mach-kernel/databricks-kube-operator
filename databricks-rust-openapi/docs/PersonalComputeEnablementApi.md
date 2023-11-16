# \PersonalComputeEnablementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_settingsdelete_personal_compute_setting**](PersonalComputeEnablementApi.md#account_settingsdelete_personal_compute_setting) | **DELETE** /api/2.0/accounts/{account_id}/settings/types/dcp_acct_enable/names/default | Delete Personal Compute setting
[**account_settingsread_personal_compute_setting**](PersonalComputeEnablementApi.md#account_settingsread_personal_compute_setting) | **GET** /api/2.0/accounts/{account_id}/settings/types/dcp_acct_enable/names/default | Get Personal Compute setting
[**account_settingsupdate_personal_compute_setting**](PersonalComputeEnablementApi.md#account_settingsupdate_personal_compute_setting) | **PATCH** /api/2.0/accounts/{account_id}/settings/types/dcp_acct_enable/names/default | Update Personal Compute setting



## account_settingsdelete_personal_compute_setting

> crate::models::AccountSettingsdeletePersonalComputeSetting200Response account_settingsdelete_personal_compute_setting(etag)
Delete Personal Compute setting

Reverts back the Personal Compute setting value to default (ON)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**etag** | [**serde_json::Value**](.md) | etag used for versioning. The response is at least as fresh as the eTag provided. This is used for optimistic concurrency control as a way to help prevent simultaneous writes of a setting overwriting each other. It is strongly suggested that systems make use of the etag in the read -> delete pattern to perform setting deletions in order to avoid race conditions. That is, get an etag from a GET request, and pass it with the DELETE request to identify the rule set version you are deleting.  | [required] |

### Return type

[**crate::models::AccountSettingsdeletePersonalComputeSetting200Response**](AccountSettingsdeletePersonalComputeSetting_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_settingsread_personal_compute_setting

> crate::models::SettingsPersonalComputeSetting account_settingsread_personal_compute_setting(etag)
Get Personal Compute setting

Gets the value of the Personal Compute setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**etag** | [**serde_json::Value**](.md) | etag used for versioning. The response is at least as fresh as the eTag provided. This is used for optimistic concurrency control as a way to help prevent simultaneous writes of a setting overwriting each other. It is strongly suggested that systems make use of the etag in the read -> delete pattern to perform setting deletions in order to avoid race conditions. That is, get an etag from a GET request, and pass it with the DELETE request to identify the rule set version you are deleting.  | [required] |

### Return type

[**crate::models::SettingsPersonalComputeSetting**](SettingsPersonalComputeSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_settingsupdate_personal_compute_setting

> crate::models::SettingsPersonalComputeSetting account_settingsupdate_personal_compute_setting(account_id, account_settingsupdate_personal_compute_setting_request)
Update Personal Compute setting

Updates the value of the Personal Compute setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**account_settingsupdate_personal_compute_setting_request** | [**AccountSettingsupdatePersonalComputeSettingRequest**](AccountSettingsupdatePersonalComputeSettingRequest.md) | Details required to update a Personal Compute setting. | [required] |

### Return type

[**crate::models::SettingsPersonalComputeSetting**](SettingsPersonalComputeSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

