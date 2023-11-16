# CatalogEffectivePrivilege

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inherited_from_name** | Option<**String**> | The full name of the object that conveys this privilege via inheritance. This field is omitted when privilege is not inherited (it's assigned to the securable itself).  | [optional]
**inherited_from_type** | Option<[**crate::models::CatalogSecurableType**](CatalogSecurableType.md)> | The type of the object that conveys this privilege via inheritance. This field is omitted when privilege is not inherited (it's assigned to the securable itself).  | [optional]
**privilege** | Option<[**crate::models::CatalogPrivilege**](CatalogPrivilege.md)> | The privilege assigned to the principal. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


