# ComputeClusterLogConf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dbfs** | Option<[**crate::models::ComputeDbfsStorageInfo**](ComputeDbfsStorageInfo.md)> | destination needs to be provided. Eg. `{ \"dbfs\" : { \"destination\" : \"dbfs:/home/cluster_log\" } }` | [optional]
**s3** | Option<[**crate::models::ComputeS3StorageInfo**](ComputeS3StorageInfo.md)> | destination and either the region or endpoint need to be provided. Eg. `{ \"s3\": { \"destination\" : \"s3://cluster_log_bucket/prefix\", \"region\" : \"us-west-2\" } }` Cluster iam role is used to access s3, please make sure the cluster iam role in `instance_profile_arn` has permission to write data to the s3 destination. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


