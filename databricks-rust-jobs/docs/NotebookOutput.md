# NotebookOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**result** | Option<**String**> | The value passed to [dbutils.notebook.exit()](https://docs.databricks.com/notebooks/notebook-workflows.html#notebook-workflows-exit). Databricks restricts this API to return the first 5 MB of the value. For a larger result, your job can store the results in a cloud storage service. This field is absent if `dbutils.notebook.exit()` was never called. | [optional]
**truncated** | Option<**bool**> | Whether or not the result was truncated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


