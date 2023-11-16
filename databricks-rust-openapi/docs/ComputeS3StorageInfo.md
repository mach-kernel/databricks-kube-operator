# ComputeS3StorageInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canned_acl** | Option<**String**> | (Optional) Set canned access control list for the logs, Eg. `bucket-owner-full-control`. If `canned_cal` is set, please make sure the cluster iam role has `s3:PutObjectAcl` permission on the destination bucket and prefix. The full list of possible canned acl can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl. Please also note that by default only the object owner gets full controls. If you are using cross account role for writing data, you may want to set `bucket-owner-full-control` to make bucket owner able to read the logs. | [optional]
**destination** | Option<**String**> | S3 destination, Eg. `s3://my-bucket/some-prefix` Note that logs will be delivered using cluster iam role, please make sure you set cluster iam role and the role has write access to the destination. Please also note that you cannot use AWS keys to deliver logs. | [optional]
**enable_encryption** | Option<**bool**> | (Optional) Flag to enable server side encryption, `false` by default. | [optional]
**encryption_type** | Option<**String**> | (Optional) The encryption type, it could be `sse-s3` or `sse-kms`. It will be used only when encryption is enabled and the default type is `sse-s3`. | [optional]
**endpoint** | Option<**String**> | S3 endpoint, Eg. `https://s3-us-west-2.amazonaws.com`. Either region or endpoint needs to be set. If both are set, endpoint will be used. | [optional]
**kms_key** | Option<**String**> | (Optional) Kms key which will be used if encryption is enabled and encryption type is set to `sse-kms`. | [optional]
**region** | Option<**String**> | S3 region, Eg. `us-west-2`. Either region or endpoint needs to be set. If both are set, endpoint will be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


