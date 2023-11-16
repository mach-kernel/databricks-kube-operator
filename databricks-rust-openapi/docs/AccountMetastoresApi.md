# \AccountMetastoresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_metastorescreate**](AccountMetastoresApi.md#account_metastorescreate) | **POST** /api/2.0/accounts/{account_id}/metastores | Create metastore
[**account_metastoresdelete**](AccountMetastoresApi.md#account_metastoresdelete) | **DELETE** /api/2.0/accounts/{account_id}/metastores/{metastore_id} | Delete a metastore
[**account_metastoresget**](AccountMetastoresApi.md#account_metastoresget) | **GET** /api/2.0/accounts/{account_id}/metastores/{metastore_id} | Get a metastore
[**account_metastoreslist**](AccountMetastoresApi.md#account_metastoreslist) | **GET** /api/2.0/accounts/{account_id}/metastores | Get all metastores associated with an account
[**account_metastoresupdate**](AccountMetastoresApi.md#account_metastoresupdate) | **PUT** /api/2.0/accounts/{account_id}/metastores/{metastore_id} | Update a metastore



## account_metastorescreate

> crate::models::CatalogAccountsMetastoreInfo account_metastorescreate(account_id, catalog_accounts_create_metastore)
Create metastore

Creates a Unity Catalog metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**catalog_accounts_create_metastore** | [**CatalogAccountsCreateMetastore**](CatalogAccountsCreateMetastore.md) | Properties of the new metastore. | [required] |

### Return type

[**crate::models::CatalogAccountsMetastoreInfo**](CatalogAccountsMetastoreInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastoresdelete

> serde_json::Value account_metastoresdelete(account_id, metastore_id, force)
Delete a metastore

Deletes a Unity Catalog metastore for an account, both specified by ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**force** | Option<**bool**> | Force deletion even if the metastore is not empty. Default is false. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastoresget

> crate::models::CatalogAccountsMetastoreInfo account_metastoresget(account_id, metastore_id)
Get a metastore

Gets a Unity Catalog metastore from an account, both specified by ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |

### Return type

[**crate::models::CatalogAccountsMetastoreInfo**](CatalogAccountsMetastoreInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastoreslist

> crate::models::CatalogListMetastoresResponse account_metastoreslist(account_id)
Get all metastores associated with an account

Gets all Unity Catalog metastores associated with an account specified by ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**crate::models::CatalogListMetastoresResponse**](CatalogListMetastoresResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastoresupdate

> crate::models::CatalogAccountsMetastoreInfo account_metastoresupdate(account_id, metastore_id, catalog_accounts_update_metastore)
Update a metastore

Updates an existing Unity Catalog metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**catalog_accounts_update_metastore** | [**CatalogAccountsUpdateMetastore**](CatalogAccountsUpdateMetastore.md) | Properties of the metastore to change. | [required] |

### Return type

[**crate::models::CatalogAccountsMetastoreInfo**](CatalogAccountsMetastoreInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

