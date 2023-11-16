# CatalogColumnInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_text** | Option<**String**> | Full data type specification as SQL/catalogString text. | [optional]
**type_json** | Option<**String**> | Full data type specification, JSON-serialized. | [optional]
**type_interval_type** | Option<**String**> | Format of IntervalType. | [optional]
**position** | Option<**i32**> | Ordinal position of column (starting at position 0). | [optional]
**name** | Option<**String**> | Name of Column. | [optional]
**type_precision** | Option<**i32**> | Digits of precision; required for DecimalTypes. | [optional]
**type_name** | Option<[**crate::models::CatalogColumnTypeName**](CatalogColumnTypeName.md)> |  | [optional]
**nullable** | Option<**bool**> | Whether field may be Null (default: true). | [optional][default to true]
**partition_index** | Option<**i32**> | Partition index for column. | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**type_scale** | Option<**i32**> | Digits to right of decimal; Required for DecimalTypes. | [optional]
**mask** | Option<[**crate::models::CatalogColumnMask**](CatalogColumnMask.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


