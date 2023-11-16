# \ServicePrincipalSecretsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_principal_secretscreate**](ServicePrincipalSecretsApi.md#service_principal_secretscreate) | **POST** /api/2.0/accounts/{account_id}/servicePrincipals/{service_principal_id}/credentials/secrets | Create service principal secret
[**service_principal_secretsdelete**](ServicePrincipalSecretsApi.md#service_principal_secretsdelete) | **DELETE** /api/2.0/accounts/{account_id}/servicePrincipals/{service_principal_id}/credentials/secrets/{secret_id}, | Delete service principal secret
[**service_principal_secretslist**](ServicePrincipalSecretsApi.md#service_principal_secretslist) | **GET** /api/2.0/accounts/{account_id}/servicePrincipals/{service_principal_id}/credentials/secrets | List service principal secrets



## service_principal_secretscreate

> crate::models::Oauth2CreateServicePrincipalSecretResponse service_principal_secretscreate(account_id, service_principal_id)
Create service principal secret

Create a secret for the given service principal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**service_principal_id** | **i64** | The service principal ID. | [required] |

### Return type

[**crate::models::Oauth2CreateServicePrincipalSecretResponse**](Oauth2CreateServicePrincipalSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_principal_secretsdelete

> service_principal_secretsdelete(account_id, service_principal_id, secret_id)
Delete service principal secret

Delete a secret from the given service principal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**service_principal_id** | [**serde_json::Value**](.md) | The service principal ID. | [required] |
**secret_id** | **String** | The secret ID. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_principal_secretslist

> crate::models::Oauth2ListServicePrincipalSecretsResponse service_principal_secretslist(account_id, service_principal_id)
List service principal secrets

List all secrets associated with the given service principal. This operation only returns information about the secrets themselves and does not include the secret values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**service_principal_id** | **i64** | The service principal ID. | [required] |

### Return type

[**crate::models::Oauth2ListServicePrincipalSecretsResponse**](Oauth2ListServicePrincipalSecretsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

