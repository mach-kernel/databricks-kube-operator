# MlDataset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**digest** | Option<**String**> | Dataset digest, Eg. an md5 hash of the dataset that uniquely identifies it within datasets of the same name. | [optional]
**name** | Option<**String**> | The name of the dataset. Eg. “my.uc.table@2” “nyc-taxi-dataset”, “fantastic-elk-3” | [optional]
**profile** | Option<**String**> | The profile of the dataset. Summary statistics for the dataset, such as the number of rows in a table, the mean / std / mode of each column in a table, or the number of elements in an array. | [optional]
**schema** | Option<**String**> | The schema of the dataset. Eg., MLflow ColSpec JSON for a dataframe, MLflow TensorSpec JSON for an ndarray, or another schema format. | [optional]
**source** | Option<**String**> | The type of the dataset source, Eg. ‘databricks-uc-table’, ‘DBFS’, ‘S3’, ... | [optional]
**source_type** | Option<**String**> | Source information for the dataset. Note that the source may not exactly reproduce the dataset if it was transformed / modified before use with MLflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


