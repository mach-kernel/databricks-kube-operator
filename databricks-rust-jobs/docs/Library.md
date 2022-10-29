# Library

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jar** | Option<**String**> | If jar, URI of the JAR to be installed. DBFS and S3 URIs are supported. For example: `{ \"jar\": \"dbfs:/mnt/databricks/library.jar\" }` or `{ \"jar\": \"s3://my-bucket/library.jar\" }`. If S3 is used, make sure the cluster has read access on the library. You may need to launch the cluster with an instance profile to access the S3 URI. | [optional]
**egg** | Option<**String**> | If egg, URI of the egg to be installed. DBFS and S3 URIs are supported. For example: `{ \"egg\": \"dbfs:/my/egg\" }` or `{ \"egg\": \"s3://my-bucket/egg\" }`. If S3 is used, make sure the cluster has read access on the library. You may need to launch the cluster with an instance profile to access the S3 URI. | [optional]
**whl** | Option<**String**> | If whl, URI of the wheel or zipped wheels to be installed. DBFS and S3 URIs are supported. For example: `{ \"whl\": \"dbfs:/my/whl\" }` or `{ \"whl\": \"s3://my-bucket/whl\" }`. If S3 is used, make sure the cluster has read access on the library. You may need to launch the cluster with an instance profile to access the S3 URI. Also the wheel file name needs to use the [correct convention](https://www.python.org/dev/peps/pep-0427/#file-format). If zipped wheels are to be installed, the file name suffix should be `.wheelhouse.zip`. | [optional]
**pypi** | Option<[**crate::models::PythonPyPiLibrary**](PythonPyPiLibrary.md)> |  | [optional]
**maven** | Option<[**crate::models::MavenLibrary**](MavenLibrary.md)> |  | [optional]
**cran** | Option<[**crate::models::RCranLibrary**](RCranLibrary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


