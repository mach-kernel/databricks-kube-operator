# CatalogFunctionParameterInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_text** | **String** | Full data type spec, SQL/catalogString text. | 
**type_json** | Option<**String**> | Full data type spec, JSON-serialized. | [optional]
**type_interval_type** | Option<**String**> | Format of IntervalType. | [optional]
**position** | **i32** | Ordinal position of column (starting at position 0). | 
**name** | **String** | Name of parameter. | 
**type_precision** | Option<**i32**> | Digits of precision; required on Create for DecimalTypes. | [optional]
**type_name** | [**crate::models::CatalogColumnTypeName**](CatalogColumnTypeName.md) |  | 
**parameter_type** | Option<[**crate::models::CatalogFunctionParameterType**](CatalogFunctionParameterType.md)> |  | [optional]
**parameter_mode** | Option<[**crate::models::CatalogFunctionParameterMode**](CatalogFunctionParameterMode.md)> |  | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**parameter_default** | Option<**String**> | Default value of the parameter. | [optional]
**type_scale** | Option<**i32**> | Digits to right of decimal; Required on Create for DecimalTypes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


