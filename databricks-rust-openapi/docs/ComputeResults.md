# ComputeResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pos** | Option<**i32**> | internal field used by SDK | [optional]
**truncated** | Option<**bool**> | true if partial results are returned. | [optional]
**schema** | Option<[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)> |  | [optional]
**summary** | Option<**String**> | The summary of the error | [optional]
**cause** | Option<**String**> | The cause of the error | [optional]
**file_name** | Option<**String**> | The image filename | [optional]
**result_type** | Option<[**crate::models::ComputeResultType**](ComputeResultType.md)> |  | [optional]
**is_json_schema** | Option<**bool**> | true if a JSON schema is returned instead of a string representation of the Hive type. | [optional]
**file_names** | Option<**Vec<String>**> |  | [optional]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


