# ComputeLibrary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cran** | Option<[**crate::models::ComputeRCranLibrary**](ComputeRCranLibrary.md)> | Specification of a CRAN library to be installed as part of the library | [optional]
**egg** | Option<**String**> | URI of the egg to be installed. Currently only DBFS and S3 URIs are supported. For example: `{ \"egg\": \"dbfs:/my/egg\" }` or `{ \"egg\": \"s3://my-bucket/egg\" }`. If S3 is used, please make sure the cluster has read access on the library. You may need to launch the cluster with an IAM role to access the S3 URI. | [optional]
**jar** | Option<**String**> | URI of the jar to be installed. Currently only DBFS and S3 URIs are supported. For example: `{ \"jar\": \"dbfs:/mnt/databricks/Libraryjar\" }` or `{ \"jar\": \"s3://my-bucket/Libraryjar\" }`. If S3 is used, please make sure the cluster has read access on the library. You may need to launch the cluster with an IAM role to access the S3 URI. | [optional]
**maven** | Option<[**crate::models::ComputeMavenLibrary**](ComputeMavenLibrary.md)> | Specification of a maven library to be installed. For example: `{ \"coordinates\": \"Orgjsoup:jsoup:1.7.2\" }` | [optional]
**pypi** | Option<[**crate::models::ComputePythonPyPiLibrary**](ComputePythonPyPiLibrary.md)> | Specification of a PyPi library to be installed. For example: `{ \"package\": \"simplejson\" }` | [optional]
**whl** | Option<**String**> | URI of the wheel to be installed. For example: `{ \"whl\": \"dbfs:/my/whl\" }` or `{ \"whl\": \"s3://my-bucket/whl\" }`. If S3 is used, please make sure the cluster has read access on the library. You may need to launch the cluster with an IAM role to access the S3 URI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


