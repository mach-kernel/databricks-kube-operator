# \DataSourcesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**data_sourceslist**](DataSourcesApi.md#data_sourceslist) | **GET** /api/2.0/preview/sql/data_sources | Get a list of SQL warehouses



## data_sourceslist

> Vec<crate::models::SqlDataSource> data_sourceslist()
Get a list of SQL warehouses

Retrieves a full list of SQL warehouses available in this workspace. All fields that appear in this API response are enumerated for clarity. However, you need only a SQL warehouse's `id` to create new queries against it. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SqlDataSource>**](SqlDataSource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

