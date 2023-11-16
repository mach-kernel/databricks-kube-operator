# ProvisioningCreateAwsKeyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_alias** | Option<**String**> | The AWS KMS key alias. | [optional]
**key_arn** | **String** | The AWS KMS key's Amazon Resource Name (ARN). Note that the key's AWS region is inferred from the ARN. | 
**reuse_key_for_cluster_volumes** | Option<**bool**> | This field applies only if the `use_cases` property includes `STORAGE`. If this is set to `true` or omitted, the key is also used to encrypt cluster EBS volumes. To not use this key also for encrypting EBS volumes, set this to `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


