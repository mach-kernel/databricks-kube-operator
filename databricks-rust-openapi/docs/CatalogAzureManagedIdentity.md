# CatalogAzureManagedIdentity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_connector_id** | **String** | The Azure resource ID of the Azure Databricks Access Connector. Use the format /subscriptions/{guid}/resourceGroups/{rg-name}/providers/MicrosoftDatabricks/accessConnectors/{connector-name}. | 
**credential_id** | Option<**String**> | The Databricks internal ID that represents this managed identity. | [optional]
**managed_identity_id** | Option<**String**> | The Azure resource ID of the managed identity. Use the format /subscriptions/{guid}/resourceGroups/{rg-name}/providers/MicrosoftManagedIdentity/userAssignedIdentities/{identity-name}. This is only available for user-assgined identities. For system-assigned identities, the access_connector_id is used to identify the identity. If this field is not provided, then we assume the AzureManagedIdentity is for a system-assigned identity.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


