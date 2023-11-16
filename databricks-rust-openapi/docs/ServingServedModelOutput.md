# ServingServedModelOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_vars** | Option<**::std::collections::HashMap<String, String>**> | An object containing a set of optional, user-specified environment variable key-value pairs used for serving this model. Note: this is an experimental feature and subject to change.  Example model environment variables that refer to Databricks secrets: `{\"OPENAI_API_KEY\": \"{{secrets/my_scope/my_key}}\", \"DATABRICKS_TOKEN\": \"{{secrets/my_scope2/my_key2}}\"}` | [optional]
**instance_profile_arn** | Option<**String**> | ARN of the instance profile that the served model will use to access AWS resources. | [optional]
**creator** | Option<**String**> | The email of the user who created the served model. | [optional]
**name** | Option<**String**> | The name of the served model. | [optional]
**model_name** | Option<**String**> | The name of the model in Databricks Model Registry or the full name of the model in Unity Catalog. | [optional]
**model_version** | Option<**String**> | The version of the model in Databricks Model Registry or Unity Catalog to be served. | [optional]
**scale_to_zero_enabled** | Option<**bool**> | Whether the compute resources for the Served Model should scale down to zero. | [optional]
**state** | Option<[**crate::models::ServingServedModelState**](ServingServedModelState.md)> | Information corresponding to the state of the Served Model. | [optional]
**creation_timestamp** | Option<**i64**> | The creation timestamp of the served model in Unix time. | [optional]
**workload_size** | Option<**String**> | The workload size of the served model. The workload size corresponds to a range of provisioned concurrency that the compute will autoscale between. A single unit of provisioned concurrency can process one request at a time. Valid workload sizes are \"Small\" (4 - 4 provisioned concurrency), \"Medium\" (8 - 16 provisioned concurrency), and \"Large\" (16 - 64 provisioned concurrency). If scale-to-zero is enabled, the lower bound of the provisioned concurrency for each workload size will be 0.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


