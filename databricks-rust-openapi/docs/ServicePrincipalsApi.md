# \ServicePrincipalsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_principalscreate**](ServicePrincipalsApi.md#service_principalscreate) | **POST** /api/2.0/preview/scim/v2/ServicePrincipals | Create a service principal
[**service_principalsdelete**](ServicePrincipalsApi.md#service_principalsdelete) | **DELETE** /api/2.0/preview/scim/v2/ServicePrincipals/{id} | Delete a service principal
[**service_principalsget**](ServicePrincipalsApi.md#service_principalsget) | **GET** /api/2.0/preview/scim/v2/ServicePrincipals/{id} | Get service principal details
[**service_principalslist**](ServicePrincipalsApi.md#service_principalslist) | **GET** /api/2.0/preview/scim/v2/ServicePrincipals | List service principals
[**service_principalspatch**](ServicePrincipalsApi.md#service_principalspatch) | **PATCH** /api/2.0/preview/scim/v2/ServicePrincipals/{id} | Update service principal details
[**service_principalsupdate**](ServicePrincipalsApi.md#service_principalsupdate) | **PUT** /api/2.0/preview/scim/v2/ServicePrincipals/{id} | Replace service principal



## service_principalscreate

> crate::models::IamServicePrincipal service_principalscreate(iam_service_principal)
Create a service principal

Creates a new service principal in the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_service_principal** | [**IamServicePrincipal**](IamServicePrincipal.md) | Properties of the new service principal. | [required] |

### Return type

[**crate::models::IamServicePrincipal**](IamServicePrincipal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_principalsdelete

> serde_json::Value service_principalsdelete(id)
Delete a service principal

Delete a single service principal in the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks workspace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_principalsget

> crate::models::IamServicePrincipal service_principalsget(id)
Get service principal details

Gets the details for a single service principal define in the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks workspace. | [required] |

### Return type

[**crate::models::IamServicePrincipal**](IamServicePrincipal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_principalslist

> crate::models::IamListServicePrincipalResponse service_principalslist(filter, attributes, excluded_attributes, start_index, count, sort_by, sort_order)
List service principals

Gets the set of service principals associated with a Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Query by which the results have to be filtered. Supported operators are equals(`eq`), contains(`co`), starts with(`sw`) and not equals(`ne`). Additionally, simple expressions can be formed using logical operators - `and` and `or`. The [SCIM RFC](https://Toolsietf.org/html/rfc7644#section-3.4.2.2) has more details but we currently only support simple expressions. |  |
**attributes** | Option<**String**> | Comma-separated list of attributes to return in response. |  |
**excluded_attributes** | Option<**String**> | Comma-separated list of attributes to exclude in response. |  |
**start_index** | Option<**i32**> | Specifies the index of the first result. First item is number 1. |  |
**count** | Option<**i32**> | Desired number of results per page. |  |
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


## service_principalspatch

> serde_json::Value service_principalspatch(id, iam_partial_update)
Update service principal details

Partially updates the details of a single service principal in the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks workspace. | [required] |
**iam_partial_update** | [**IamPartialUpdate**](IamPartialUpdate.md) | Operations to be applied on service principal information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_principalsupdate

> serde_json::Value service_principalsupdate(id, iam_service_principal)
Replace service principal

Updates the details of a single service principal.   This action replaces the existing service principal with the same name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a service principal in the Databricks workspace. | [required] |
**iam_service_principal** | [**IamServicePrincipal**](IamServicePrincipal.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

