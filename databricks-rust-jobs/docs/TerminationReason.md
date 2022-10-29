# TerminationReason

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<[**crate::models::TerminationCode**](TerminationCode.md)> |  | [optional]
**r#type** | Option<[**crate::models::TerminationType**](TerminationType.md)> |  | [optional]
**parameters** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object with additional information about why a cluster was terminated. The object keys are one of `TerminationParameter` and the value is the termination information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


