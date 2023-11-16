# ProvisioningUpsertPrivateAccessSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_vpc_endpoint_ids** | Option<[**Vec<String>**](String.md)> |  | [optional]
**private_access_level** | Option<[**crate::models::ProvisioningPrivateAccessLevel**](ProvisioningPrivateAccessLevel.md)> |  | [optional]
**private_access_settings_name** | **String** | The human-readable name of the private access settings object. | 
**public_access_enabled** | Option<**bool**> | Determines if the workspace can be accessed over public internet. For fully private workspaces, you can optionally specify `false`, but only if you implement both the front-end and the back-end PrivateLink connections. Otherwise, specify `true`, which means that public access is enabled. | [optional][default to false]
**region** | **String** | The cloud region for workspaces associated with this private access settings object. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


