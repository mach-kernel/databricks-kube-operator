# S3StorageInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<**String**> | S3 destination. For example: `s3://my-bucket/some-prefix` You must configure the cluster with an instance profile and the instance profile must have write access to the destination. You _cannot_ use AWS keys. | [optional]
**region** | Option<**String**> | S3 region. For example: `us-west-2`. Either region or endpoint must be set. If both are set, endpoint is used. | [optional]
**endpoint** | Option<**String**> | S3 endpoint. For example: `https://s3-us-west-2.amazonaws.com`. Either region or endpoint must be set. If both are set, endpoint is used. | [optional]
**enable_encryption** | Option<**bool**> | (Optional)Enable server side encryption, `false` by default. | [optional]
**encryption_type** | Option<**String**> | (Optional) The encryption type, it could be `sse-s3` or `sse-kms`. It is used only when encryption is enabled and the default type is `sse-s3`. | [optional]
**kms_key** | Option<**String**> | (Optional) KMS key used if encryption is enabled and encryption type is set to `sse-kms`. | [optional]
**canned_acl** | Option<**String**> | (Optional) Set canned access control list. For example: `bucket-owner-full-control`. If canned_acl is set, the cluster instance profile must have `s3:PutObjectAcl` permission on the destination bucket and prefix. The full list of possible canned ACLs can be found at <https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl>. By default only the object owner gets full control. If you are using cross account role for writing data, you may want to set `bucket-owner-full-control` to make bucket owner able to read the logs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


