# ProvisioningAwsKeyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_alias** | Option<**String**> | The AWS KMS key alias. | [optional]
**key_arn** | **String** | The AWS KMS key's Amazon Resource Name (ARN). | 
**key_region** | **String** | The AWS KMS key region. | 
**reuse_key_for_cluster_volumes** | Option<**bool**> | This field applies only if the `use_cases` property includes `STORAGE`. If this is set to `true` or omitted, the key is also used to encrypt cluster EBS volumes. If you do not want to use this key for encrypting EBS volumes, set to `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


