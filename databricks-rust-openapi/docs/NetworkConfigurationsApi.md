# \NetworkConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**networkscreate**](NetworkConfigurationsApi.md#networkscreate) | **POST** /api/2.0/accounts/{account_id}/networks | Create network configuration
[**networksdelete**](NetworkConfigurationsApi.md#networksdelete) | **DELETE** /api/2.0/accounts/{account_id}/networks/{network_id} | Delete a network configuration
[**networksget**](NetworkConfigurationsApi.md#networksget) | **GET** /api/2.0/accounts/{account_id}/networks/{network_id} | Get a network configuration
[**networkslist**](NetworkConfigurationsApi.md#networkslist) | **GET** /api/2.0/accounts/{account_id}/networks | Get all network configurations



## networkscreate

> crate::models::ProvisioningNetwork networkscreate(provisioning_create_network_request, x_databricks_gcp_sa_access_token)
Create network configuration

Creates a Databricks network configuration that represents an VPC and its resources. The VPC will be used for new Databricks clusters. This requires a pre-existing VPC and subnets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provisioning_create_network_request** | [**ProvisioningCreateNetworkRequest**](ProvisioningCreateNetworkRequest.md) | Properties of the new network configuration. | [required] |
**x_databricks_gcp_sa_access_token** | Option<[**serde_json::Value**](.md)> | The Google Cloud access token of the caller. For details about this access token, see [Authentication using Open ID Connect (OIDC) tokens](https://Docsgcp.databricks.com/dev-tools/api/latest/authentication-oidc.html). |  |

### Return type

[**crate::models::ProvisioningNetwork**](ProvisioningNetwork.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## networksdelete

> serde_json::Value networksdelete(network_id, x_databricks_gcp_sa_access_token)
Delete a network configuration

Deletes a Databricks network configuration, which represents a cloud VPC and its resources. You cannot delete a network that is associated with a workspace.  This operation is available only if your account is on the E2 version of the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | [**serde_json::Value**](.md) | Databricks Account API network configuration ID. | [required] |
**x_databricks_gcp_sa_access_token** | Option<[**serde_json::Value**](.md)> | The Google Cloud access token of the caller. For details about this access token, see [Authentication using Open ID Connect (OIDC) tokens](https://Docsgcp.databricks.com/dev-tools/api/latest/authentication-oidc.html). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## networksget

> crate::models::ProvisioningNetwork networksget(account_id, network_id)
Get a network configuration

Gets a Databricks network configuration, which represents a cloud VPC and its resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**network_id** | [**serde_json::Value**](.md) | Databricks Account API network configuration ID. | [required] |

### Return type

[**crate::models::ProvisioningNetwork**](ProvisioningNetwork.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## networkslist

> Vec<crate::models::ProvisioningNetwork> networkslist(account_id)
Get all network configurations

Gets a list of all Databricks network configurations for an account, specified by ID.  This operation is available only if your account is on the E2 version of the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**Vec<crate::models::ProvisioningNetwork>**](ProvisioningNetwork.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

