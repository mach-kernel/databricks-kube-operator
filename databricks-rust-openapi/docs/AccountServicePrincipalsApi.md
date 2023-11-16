# \AccountServicePrincipalsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_service_principalscreate**](AccountServicePrincipalsApi.md#account_service_principalscreate) | **POST** /api/2.0/accounts/{account_id}/scim/v2/ServicePrincipals | Create a service principal
[**account_service_principalsdelete**](AccountServicePrincipalsApi.md#account_service_principalsdelete) | **DELETE** /api/2.0/accounts/{account_id}/scim/v2/ServicePrincipals/{id} | Delete a service principal
[**account_service_principalsget**](AccountServicePrincipalsApi.md#account_service_principalsget) | **GET** /api/2.0/accounts/{account_id}/scim/v2/ServicePrincipals/{id} | Get service principal details
[**account_service_principalslist**](AccountServicePrincipalsApi.md#account_service_principalslist) | **GET** /api/2.0/accounts/{account_id}/scim/v2/ServicePrincipals | List service principals
[**account_service_principalspatch**](AccountServicePrincipalsApi.md#account_service_principalspatch) | **PATCH** /api/2.0/accounts/{account_id}/scim/v2/ServicePrincipals/{id} | Update service principal details
[**account_service_principalsupdate**](AccountServicePrincipalsApi.md#account_service_principalsupdate) | **PUT** /api/2.0/accounts/{account_id}/scim/v2/ServicePrincipals/{id} | Replace service principal



## account_service_principalscreate

> crate::models::IamServicePrincipal account_service_principalscreate(account_id, iam_service_principal)
Create a service principal

Creates a new service principal in the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**iam_service_principal** | [**IamServicePrincipal**](IamServicePrincipal.md) | Properties of the new service principal. | [required] |

### Return type

[**crate::models::IamServicePrincipal**](IamServicePrincipal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_service_principalsdelete

> serde_json::Value account_service_principalsdelete(account_id, id)
Delete a service principal

Delete a single service principal in the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks account. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_service_principalsget

> crate::models::IamServicePrincipal account_service_principalsget(account_id, id)
Get service principal details

Gets the details for a single service principal define in the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks account. | [required] |

### Return type

[**crate::models::IamServicePrincipal**](IamServicePrincipal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_service_principalslist

> crate::models::IamListServicePrincipalResponse account_service_principalslist(account_id, filter, attributes, excluded_attributes, start_index, count, sort_by, sort_order)
List service principals

Gets the set of service principals associated with a Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**filter** | Option<**String**> | Query by which the results have to be filtered. Supported operators are equals(`eq`), contains(`co`), starts with(`sw`) and not equals(`ne`). Additionally, simple expressions can be formed using logical operators - `and` and `or`. The [SCIM RFC](https://Toolsietf.org/html/rfc7644#section-3.4.2.2) has more details but we currently only support simple expressions. |  |
**attributes** | Option<**String**> | Comma-separated list of attributes to return in response. |  |
**excluded_attributes** | Option<**String**> | Comma-separated list of attributes to exclude in response. |  |
**start_index** | Option<**i32**> | Specifies the index of the first result. First item is number 1. |  |
**count** | Option<**i32**> | Desired number of results per page. Default is 10000. |  |
**sort_by** | Option<**String**> | Attribute to sort the results. |  |
**sort_order** | Option<**String**> | The order to sort the results. |  |

### Return type

[**crate::models::IamListServicePrincipalResponse**](IamListServicePrincipalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_service_principalspatch

> serde_json::Value account_service_principalspatch(account_id, id, iam_partial_update)
Update service principal details

Partially updates the details of a single service principal in the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks account. | [required] |
**iam_partial_update** | [**IamPartialUpdate**](IamPartialUpdate.md) | Operations to be applied on service principal information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_service_principalsupdate

> serde_json::Value account_service_principalsupdate(account_id, id, iam_service_principal)
Replace service principal

Updates the details of a single service principal.   This action replaces the existing service principal with the same name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks account. | [required] |
**iam_service_principal** | [**IamServicePrincipal**](IamServicePrincipal.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

