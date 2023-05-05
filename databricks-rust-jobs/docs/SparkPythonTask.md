# SparkPythonTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**python_file** | **String** | The Python file to be executed. Cloud file URIs (such as dbfs:/, s3:/, adls:/, gcs:/) and workspace paths are supported. For python files stored in the Databricks workspace, the path must be absolute and begin with `/`. This field is required. | 
**parameters** | Option<**Vec<String>**> | Command line parameters passed to the Python file.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


