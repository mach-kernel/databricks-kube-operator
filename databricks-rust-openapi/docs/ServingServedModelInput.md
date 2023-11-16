# ServingServedModelInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_vars** | Option<**::std::collections::HashMap<String, String>**> | An object containing a set of optional, user-specified environment variable key-value pairs used for serving this model. Note: this is an experimental feature and subject to change.  Example model environment variables that refer to Databricks secrets: `{\"OPENAI_API_KEY\": \"{{secrets/my_scope/my_key}}\", \"DATABRICKS_TOKEN\": \"{{secrets/my_scope2/my_key2}}\"}` | [optional]
**instance_profile_arn** | Option<**String**> | ARN of the instance profile that the served model will use to access AWS resources. | [optional]
**model_name** | **String** | The name of the model in Databricks Model Registry to be served or if the model resides in Unity Catalog, the full name of model,  in the form of __catalog_name__.__schema_name__.__model_name__.  | 
**model_version** | **String** | The version of the model in Databricks Model Registry or Unity Catalog to be served. | 
**name** | Option<**String**> | The name of a served model. It must be unique across an endpoint. If not specified, this field will default to <model-name>-<model-version>. A served model name can consist of alphanumeric characters, dashes, and underscores.  | [optional]
**scale_to_zero_enabled** | **bool** | Whether the compute resources for the served model should scale down to zero. | 
**workload_size** | **String** | The workload size of the served model. The workload size corresponds to a range of provisioned concurrency that the compute will autoscale between. A single unit of provisioned concurrency can process one request at a time. Valid workload sizes are \"Small\" (4 - 4 provisioned concurrency), \"Medium\" (8 - 16 provisioned concurrency), and \"Large\" (16 - 64 provisioned concurrency). If scale-to-zero is enabled, the lower bound of the provisioned concurrency for each workload size will be 0.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


