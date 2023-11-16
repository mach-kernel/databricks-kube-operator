# CatalogValidateStorageCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_iam_role** | Option<[**crate::models::CatalogAwsIamRole**](CatalogAwsIamRole.md)> | The AWS IAM role configuration. | [optional]
**azure_managed_identity** | Option<[**crate::models::CatalogAzureManagedIdentity**](CatalogAzureManagedIdentity.md)> | The Azure managed identity configuration. | [optional]
**azure_service_principal** | Option<[**crate::models::CatalogAzureServicePrincipal**](CatalogAzureServicePrincipal.md)> | The Azure service principal configuration. | [optional]
**databricks_gcp_service_account** | Option<[**serde_json::Value**](.md)> |  | [optional]
**external_location_name** | Option<**String**> | The name of an existing external location to validate. | [optional]
**read_only** | Option<**bool**> | Whether the storage credential is only usable for read operations. | [optional]
**storage_credential_name** | Option<[**serde_json::Value**](.md)> | The name of the storage credential to validate. | [optional]
**url** | Option<**String**> | The external location url to validate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


