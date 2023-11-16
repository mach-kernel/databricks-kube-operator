# CatalogCreateFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**parameter_style** | **String** | Function parameter style. **S** is the value for SQL. | 
**sql_data_access** | **String** | Function SQL data access. | 
**catalog_name** | **String** | Name of parent catalog. | 
**data_type** | [**crate::models::CatalogColumnTypeName**](CatalogColumnTypeName.md) | Scalar function return data type. | 
**routine_body** | **String** | Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.  | 
**is_null_call** | **bool** | Function null call. | 
**sql_path** | Option<**String**> | List of schemes whose objects can be referenced without qualification. | [optional]
**specific_name** | **String** | Specific name of the function; Reserved for future use. | 
**routine_dependencies** | [**Vec<crate::models::CatalogDependency>**](CatalogDependency.md) |  | 
**input_params** | [**Vec<crate::models::CatalogFunctionParameterInfo>**](CatalogFunctionParameterInfo.md) |  | 
**schema_name** | **String** | Name of parent schema relative to its parent catalog. | 
**security_type** | **String** | Function security type. | 
**external_name** | Option<**String**> | External function name. | [optional]
**name** | **String** | Name of function, relative to parent schema. | 
**full_data_type** | **String** | Pretty printed function data type. | 
**is_deterministic** | **bool** | Whether the function is deterministic. | 
**routine_definition** | **String** | Function body. | 
**return_params** | [**Vec<crate::models::CatalogFunctionParameterInfo>**](CatalogFunctionParameterInfo.md) |  | 
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**external_language** | Option<**String**> | External function language. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


