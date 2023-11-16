# ComputeInstanceProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iam_role_arn** | Option<**String**> | The AWS IAM role ARN of the role associated with the instance profile. This field is required if your role name and instance profile name do not match and you want to use the instance profile with [Databricks SQL Serverless](https://Docsdatabricks.com/sql/admin/serverless.html).  Otherwise, this field is optional.  | [optional]
**instance_profile_arn** | **String** | The AWS ARN of the instance profile to register with Databricks. This field is required. | 
**is_meta_instance_profile** | Option<**bool**> | Boolean flag indicating whether the instance profile should only be used in credential passthrough scenarios. If true, it means the instance profile contains an meta IAM role which could assume a wide range of roles. Therefore it should always be used with authorization. This field is optional, the default value is `false`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


