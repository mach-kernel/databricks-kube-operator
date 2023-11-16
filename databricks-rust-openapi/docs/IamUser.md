# IamUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entitlements** | Option<[**Vec<crate::models::IamComplexValue>**](IamComplexValue.md)> |  | [optional]
**display_name** | Option<**String**> | String that represents a concatenation of given and family names. For example `John Smith`. | [optional]
**id** | Option<**String**> | Databricks user ID. | [optional]
**name** | Option<[**crate::models::IamName**](IamName.md)> |  | [optional]
**user_name** | Option<**String**> | Email address of the Databricks user. | [optional]
**emails** | Option<[**Vec<crate::models::IamComplexValue>**](IamComplexValue.md)> |  | [optional]
**active** | Option<**bool**> | If this user is active | [optional]
**roles** | Option<[**serde_json::Value**](.md)> |  | [optional]
**groups** | Option<[**serde_json::Value**](.md)> |  | [optional]
**external_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


