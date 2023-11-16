# JobsSparkPythonTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parameters** | Option<**Vec<String>**> |  | [optional]
**python_file** | **String** | The Python file to be executed. Cloud file URIs (such as dbfs:/, s3:/, adls:/, gcs:/) and workspace paths are supported. For python files stored in the Databricks workspace, the path must be absolute and begin with `/`. For files stored in a remote repository, the path must be relative. This field is required. | 
**source** | Option<[**crate::models::JobsSource**](JobsSource.md)> | Optional location type of the Python file. When set to `WORKSPACE` or not specified, the file will be retrieved from the local <Databricks> workspace or cloud location (if the `python_file` has a URI format). When set to `GIT`, the Python file will be retrieved from a Git repository defined in `git_source`.  * `WORKSPACE`: The Python file is located in a <Databricks> workspace or at a cloud filesystem URI. * `GIT`: The Python file is located in a remote Git repository.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


