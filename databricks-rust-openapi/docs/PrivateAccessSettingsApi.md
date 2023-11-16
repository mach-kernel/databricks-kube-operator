# \PrivateAccessSettingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**private_accesscreate**](PrivateAccessSettingsApi.md#private_accesscreate) | **POST** /api/2.0/accounts/{account_id}/private-access-settings | Create private access settings
[**private_accessdelete**](PrivateAccessSettingsApi.md#private_accessdelete) | **DELETE** /api/2.0/accounts/{account_id}/private-access-settings/{private_access_settings_id} | Delete a private access settings object
[**private_accessget**](PrivateAccessSettingsApi.md#private_accessget) | **GET** /api/2.0/accounts/{account_id}/private-access-settings/{private_access_settings_id} | Get a private access settings object
[**private_accesslist**](PrivateAccessSettingsApi.md#private_accesslist) | **GET** /api/2.0/accounts/{account_id}/private-access-settings | Get all private access settings objects
[**private_accessreplace**](PrivateAccessSettingsApi.md#private_accessreplace) | **PUT** /api/2.0/accounts/{account_id}/private-access-settings/{private_access_settings_id} | Replace private access settings



## private_accesscreate

> crate::models::ProvisioningPrivateAccessSettings private_accesscreate(account_id, provisioning_upsert_private_access_settings_request)
Create private access settings

Creates a private access settings object, which specifies how your workspace is accessed over [AWS PrivateLink](https://Awsamazon.com/privatelink). To use AWS PrivateLink, a workspace must have a private access settings object referenced by ID in the workspace's `private_access_settings_id` property.  You can share one private access settings with multiple workspaces in a single account. However, private access settings are specific to AWS regions, so only workspaces in the same AWS region can use a given private access settings object.  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**provisioning_upsert_private_access_settings_request** | [**ProvisioningUpsertPrivateAccessSettingsRequest**](ProvisioningUpsertPrivateAccessSettingsRequest.md) | Properties of the new private access settings object. | [required] |

### Return type

[**crate::models::ProvisioningPrivateAccessSettings**](ProvisioningPrivateAccessSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_accessdelete

> serde_json::Value private_accessdelete(account_id, private_access_settings_id)
Delete a private access settings object

Deletes a private access settings object, which determines how your workspace is accessed over [AWS PrivateLink](https://Awsamazon.com/privatelink).  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**private_access_settings_id** | [**serde_json::Value**](.md) | Databricks Account API private access settings ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_accessget

> crate::models::ProvisioningPrivateAccessSettings private_accessget(account_id, private_access_settings_id)
Get a private access settings object

Gets a private access settings object, which specifies how your workspace is accessed over [AWS PrivateLink](https://Awsamazon.com/privatelink).  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**private_access_settings_id** | [**serde_json::Value**](.md) | Databricks Account API private access settings ID. | [required] |

### Return type

[**crate::models::ProvisioningPrivateAccessSettings**](ProvisioningPrivateAccessSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_accesslist

> Vec<crate::models::ProvisioningPrivateAccessSettings> private_accesslist(account_id)
Get all private access settings objects

Gets a list of all private access settings objects for an account, specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**Vec<crate::models::ProvisioningPrivateAccessSettings>**](ProvisioningPrivateAccessSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_accessreplace

> serde_json::Value private_accessreplace(account_id, private_access_settings_id, provisioning_upsert_private_access_settings_request)
Replace private access settings

Updates an existing private access settings object, which specifies how your workspace is accessed over [AWS PrivateLink](https://Awsamazon.com/privatelink). To use AWS PrivateLink, a workspace must have a private access settings object referenced by ID in the workspace's `private_access_settings_id` property.  This operation completely overwrites your existing private access settings object attached to your workspaces. All workspaces attached to the private access settings are affected by any change. If `public_access_enabled`, `private_access_level`, or `allowed_vpc_endpoint_ids` are updated, effects of these changes might take several minutes to propagate to the workspace API.  You can share one private access settings object with multiple workspaces in a single account. However, private access settings are specific to AWS regions, so only workspaces in the same AWS region can use a given private access settings object.  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**private_access_settings_id** | [**serde_json::Value**](.md) | Databricks Account API private access settings ID. | [required] |
**provisioning_upsert_private_access_settings_request** | [**ProvisioningUpsertPrivateAccessSettingsRequest**](ProvisioningUpsertPrivateAccessSettingsRequest.md) | Properties of the new private access settings object. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

