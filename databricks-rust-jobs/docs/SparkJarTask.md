# SparkJarTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**main_class_name** | Option<**String**> | The full name of the class containing the main method to be executed. This class must be contained in a JAR provided as a library.  The code must use `SparkContext.getOrCreate` to obtain a Spark context; otherwise, runs of the job fail. | [optional]
**parameters** | Option<**Vec<String>**> | Parameters passed to the main method.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs. | [optional]
**jar_uri** | Option<**String**> | Deprecated since 04/2016\\. Provide a `jar` through the `libraries` field instead. For an example, see [Create](https://docs.databricks.com/dev-tools/api/latest/jobs.html#operation/JobsCreate). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


