# \TableConstraintsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**table_constraintscreate**](TableConstraintsApi.md#table_constraintscreate) | **POST** /api/2.1/unity-catalog/constraints | Create a table constraint
[**table_constraintsdelete**](TableConstraintsApi.md#table_constraintsdelete) | **DELETE** /api/2.1/unity-catalog/constraints/{full_name} | Delete a table constraint



## table_constraintscreate

> crate::models::CatalogTableConstraint table_constraintscreate(catalog_create_table_constraint)
Create a table constraint

Creates a new table constraint.  For the table constraint creation to succeed, the user must satisfy both of these conditions: - the user must have the **USE_CATALOG** privilege on the table's parent catalog,   the **USE_SCHEMA** privilege on the table's parent schema, and be the owner of the table. - if the new constraint is a __ForeignKeyConstraint__,   the user must have the **USE_CATALOG** privilege on the referenced parent table's catalog,   the **USE_SCHEMA** privilege on the referenced parent table's schema,   and be the owner of the referenced parent table. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_table_constraint** | Option<[**CatalogCreateTableConstraint**](CatalogCreateTableConstraint.md)> |  |  |

### Return type

[**crate::models::CatalogTableConstraint**](CatalogTableConstraint.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## table_constraintsdelete

> serde_json::Value table_constraintsdelete(constraint_name, cascade, full_name)
Delete a table constraint

Deletes a table constraint.  For the table constraint deletion to succeed, the user must satisfy both of these conditions: - the user must have the **USE_CATALOG** privilege on the table's parent catalog,   the **USE_SCHEMA** privilege on the table's parent schema, and be the owner of the table. - if __cascade__ argument is **true**, the user must have the following permissions on all of the child tables:   the **USE_CATALOG** privilege on the table's catalog,   the **USE_SCHEMA** privilege on the table's schema,   and be the owner of the table. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**constraint_name** | **String** | The name of the constraint to delete. | [required] |
**cascade** | **bool** | If true, try deleting all child constraints of the current constraint. If false, reject this operation if the current constraint has any child constraints.  | [required] |[default to false]
**full_name** | **String** | Full name of the table referenced by the constraint. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

