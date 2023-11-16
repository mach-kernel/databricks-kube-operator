# CatalogFunctionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**owner** | Option<**String**> | Username of current owner of function. | [optional]
**parameter_style** | Option<**String**> | Function parameter style. **S** is the value for SQL. | [optional]
**sql_data_access** | Option<**String**> | Function SQL data access. | [optional]
**catalog_name** | Option<**String**> | Name of parent catalog. | [optional]
**data_type** | Option<[**crate::models::CatalogColumnTypeName**](CatalogColumnTypeName.md)> | Scalar function return data type. | [optional]
**routine_body** | Option<**String**> | Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.  | [optional]
**full_name** | Option<**String**> | Full name of function, in form of __catalog_name__.__schema_name__.__function__name__ | [optional][readonly]
**metastore_id** | Option<**String**> | Unique identifier of parent metastore. | [optional][readonly]
**is_null_call** | Option<**bool**> | Function null call. | [optional]
**sql_path** | Option<**String**> | List of schemes whose objects can be referenced without qualification. | [optional]
**specific_name** | Option<**String**> | Specific name of the function; Reserved for future use. | [optional]
**created_by** | Option<**String**> | Username of function creator. | [optional][readonly]
**routine_dependencies** | Option<[**Vec<crate::models::CatalogDependency>**](CatalogDependency.md)> |  | [optional]
**input_params** | Option<[**Vec<crate::models::CatalogFunctionParameterInfo>**](CatalogFunctionParameterInfo.md)> |  | [optional]
**schema_name** | Option<**String**> | Name of parent schema relative to its parent catalog. | [optional]
**security_type** | Option<**String**> | Function security type. | [optional]
**external_name** | Option<**String**> | External function name. | [optional]
**name** | Option<**String**> | Name of function, relative to parent schema. | [optional]
**updated_at** | Option<**i64**> | Time at which this function was created, in epoch milliseconds. | [optional][readonly]
**full_data_type** | Option<**String**> | Pretty printed function data type. | [optional]
**is_deterministic** | Option<**bool**> | Whether the function is deterministic. | [optional]
**routine_definition** | Option<**String**> | Function body. | [optional]
**created_at** | Option<**i64**> | Time at which this function was created, in epoch milliseconds. | [optional][readonly]
**return_params** | Option<[**Vec<crate::models::CatalogFunctionParameterInfo>**](CatalogFunctionParameterInfo.md)> |  | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified function. | [optional][readonly]
**external_language** | Option<**String**> | External function language. | [optional]
**function_id** | Option<**String**> | Id of Function, relative to parent schema. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


