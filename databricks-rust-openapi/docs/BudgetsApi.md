# \BudgetsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budgetscreate**](BudgetsApi.md#budgetscreate) | **POST** /api/2.0/accounts/{account_id}/budget | Create a new budget
[**budgetsdelete**](BudgetsApi.md#budgetsdelete) | **DELETE** /api/2.0/accounts/{account_id}/budget/{budget_id} | Delete budget
[**budgetsget**](BudgetsApi.md#budgetsget) | **GET** /api/2.0/accounts/{account_id}/budget/{budget_id} | Get budget and its status
[**budgetslist**](BudgetsApi.md#budgetslist) | **GET** /api/2.0/accounts/{account_id}/budget | Get all budgets
[**budgetsupdate**](BudgetsApi.md#budgetsupdate) | **PATCH** /api/2.0/accounts/{account_id}/budget/{budget_id} | Modify budget



## budgetscreate

> crate::models::BillingWrappedBudgetWithStatus budgetscreate(account_id, billing_wrapped_budget)
Create a new budget

Creates a new budget in the specified account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**billing_wrapped_budget** | [**BillingWrappedBudget**](BillingWrappedBudget.md) | Properties of the new budget | [required] |

### Return type

[**crate::models::BillingWrappedBudgetWithStatus**](BillingWrappedBudgetWithStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budgetsdelete

> serde_json::Value budgetsdelete(account_id, budget_id)
Delete budget

Deletes the budget specified by its UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**budget_id** | [**serde_json::Value**](.md) | Budget ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budgetsget

> crate::models::BillingWrappedBudgetWithStatus budgetsget(account_id, budget_id)
Get budget and its status

Gets the budget specified by its UUID, including noncumulative status for each day that the budget is configured to include.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**budget_id** | [**serde_json::Value**](.md) | Budget ID | [required] |

### Return type

[**crate::models::BillingWrappedBudgetWithStatus**](BillingWrappedBudgetWithStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budgetslist

> crate::models::BillingBudgetList budgetslist(account_id)
Get all budgets

Gets all budgets associated with this account, including noncumulative status for each day that the budget is configured to include.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**crate::models::BillingBudgetList**](BillingBudgetList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budgetsupdate

> serde_json::Value budgetsupdate(account_id, budget_id, billing_wrapped_budget)
Modify budget

Modifies a budget in this account. Budget properties are completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**budget_id** | [**serde_json::Value**](.md) | Budget ID | [required] |
**billing_wrapped_budget** | [**BillingWrappedBudget**](BillingWrappedBudget.md) | Properties to set the budget to | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

