# CatalogCreateStorageCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_iam_role** | Option<[**crate::models::CatalogAwsIamRole**](CatalogAwsIamRole.md)> | The AWS IAM role configuration. | [optional]
**azure_managed_identity** | Option<[**crate::models::CatalogAzureManagedIdentity**](CatalogAzureManagedIdentity.md)> | The Azure managed identity configuration. | [optional]
**azure_service_principal** | Option<[**crate::models::CatalogAzureServicePrincipal**](CatalogAzureServicePrincipal.md)> | The Azure service principal configuration. | [optional]
**comment** | Option<**String**> | Comment associated with the credential. | [optional]
**databricks_gcp_service_account** | Option<[**serde_json::Value**](.md)> |  | [optional]
**name** | **String** | The credential name. The name must be unique within the metastore. | 
**read_only** | Option<**bool**> | Whether the storage credential is only usable for read operations. | [optional]
**skip_validation** | Option<**bool**> | Supplying true to this argument skips validation of the created credential. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


