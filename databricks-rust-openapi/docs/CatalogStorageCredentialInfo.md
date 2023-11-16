# CatalogStorageCredentialInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | Username of current owner of credential. | [optional]
**databricks_gcp_service_account** | Option<[**crate::models::CatalogDatabricksGcpServiceAccountResponse**](CatalogDatabricksGcpServiceAccountResponse.md)> | The <Databricks> managed GCP service account configuration. | [optional]
**azure_service_principal** | Option<[**crate::models::CatalogAzureServicePrincipal**](CatalogAzureServicePrincipal.md)> | The Azure service principal configuration. | [optional]
**metastore_id** | Option<**String**> | Unique identifier of parent metastore. | [optional][readonly]
**id** | Option<**String**> | The unique identifier of the credential. | [optional][readonly]
**created_by** | Option<**String**> | Username of credential creator. | [optional][readonly]
**read_only** | Option<**bool**> | Whether the storage credential is only usable for read operations. | [optional]
**name** | Option<**String**> | The credential name. The name must be unique within the metastore. | [optional]
**updated_at** | Option<**i64**> | Time at which this credential was last modified, in epoch milliseconds. | [optional][readonly]
**used_for_managed_storage** | Option<**bool**> | Whether this credential is the current metastore's root storage credential. | [optional][readonly]
**azure_managed_identity** | Option<[**crate::models::CatalogAzureManagedIdentity**](CatalogAzureManagedIdentity.md)> | The Azure managed identity configuration. | [optional]
**aws_iam_role** | Option<[**crate::models::CatalogAwsIamRole**](CatalogAwsIamRole.md)> | The AWS IAM role configuration. | [optional]
**created_at** | Option<**i64**> | Time at which this Credential was created, in epoch milliseconds. | [optional][readonly]
**comment** | Option<**String**> | Comment associated with the credential. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified the credential. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


