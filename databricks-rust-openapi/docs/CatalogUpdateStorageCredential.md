# CatalogUpdateStorageCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | Username of current owner of credential. | [optional]
**databricks_gcp_service_account** | Option<[**serde_json::Value**](.md)> |  | [optional]
**azure_service_principal** | Option<[**crate::models::CatalogAzureServicePrincipal**](CatalogAzureServicePrincipal.md)> | The Azure service principal configuration. | [optional]
**force** | Option<**bool**> | Force update even if there are dependent external locations or external tables. | [optional]
**read_only** | Option<**bool**> | Whether the storage credential is only usable for read operations. | [optional]
**name** | Option<**String**> | The credential name. The name must be unique within the metastore. | [optional]
**skip_validation** | Option<**bool**> | Supplying true to this argument skips validation of the updated credential. | [optional][default to false]
**azure_managed_identity** | Option<[**crate::models::CatalogAzureManagedIdentity**](CatalogAzureManagedIdentity.md)> | The Azure managed identity configuration. | [optional]
**aws_iam_role** | Option<[**crate::models::CatalogAwsIamRole**](CatalogAwsIamRole.md)> | The AWS IAM role configuration. | [optional]
**comment** | Option<**String**> | Comment associated with the credential. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


