# BillingBudget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alerts** | Option<[**Vec<crate::models::BillingBudgetAlert>**](BillingBudgetAlert.md)> |  | [optional]
**end_date** | Option<[**String**](string.md)> | Optional end date of the budget. | [optional]
**filter** | **String** |  SQL-like filter expression with workspaceId, SKU and tag. Usage in your account that matches this expression will be counted in this budget.  Supported properties on left-hand side of comparison:  * `workspaceId` - the ID of the workspace  * `sku` - SKU of the cluster, Eg. `STANDARD_ALL_PURPOSE_COMPUTE`   * `tag.tagName`, `tag.'tag name'` - tag of the cluster   Supported comparison operators:  * `=` - equal   * `!=` - not equal   Supported logical operators: `AND`, `OR`.  Examples:  * `workspaceId=123 OR (sku='STANDARD_ALL_PURPOSE_COMPUTE' AND tag.'my tag'='my value')`  * `workspaceId!=456`  * `sku='STANDARD_ALL_PURPOSE_COMPUTE' OR sku='PREMIUM_ALL_PURPOSE_COMPUTE'`  * `tag.name1='value1' AND tag.name2='value2'`   | 
**name** | **String** | Human-readable name of the budget. | 
**period** | **String** |  Period length in years, months, weeks and/or days.  Examples: `1 month`, `30 days`, `1 year, 2 months, 1 week, 2 days`   | 
**start_date** | [**String**](string.md) | Start date of the budget period calculation. | 
**target_amount** | **String** | Target amount of the budget per period in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


