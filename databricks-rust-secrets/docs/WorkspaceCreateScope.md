# WorkspaceCreateScope

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backend_azure_keyvault** | Option<[**models::WorkspaceAzureKeyVaultSecretScopeMetadata**](WorkspaceAzureKeyVaultSecretScopeMetadata.md)> | The metadata for the secret scope if the type is `AZURE_KEYVAULT` | [optional]
**initial_manage_principal** | Option<**String**> | The principal that is initially granted `MANAGE` permission to the created scope. | [optional]
**scope** | **String** | Scope name requested by the user. Scope names are unique. | 
**scope_backend_type** | Option<[**models::WorkspaceScopeBackendType**](WorkspaceScopeBackendType.md)> | The backend type the scope will be created with. If not specified, will default to `DATABRICKS` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


