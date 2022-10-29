# DbtTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_directory** | Option<**String**> | Optional (relative) path to the project directory, if no value is provided, the root of the git repository is used. | [optional]
**commands** | **Vec<String>** | A list of dbt commands to execute. All commands must start with `dbt`. This parameter must not be empty. A maximum of up to 10 commands can be provided. | 
**schema** | Option<**String**> | Optional schema to write to. This parameter is only used when a warehouse_id is also provided. If not provided, the `default` schema is used. | [optional]
**warehouse_id** | Option<**String**> | ID of the SQL warehouse to connect to. If provided, we automatically generate and provide the profile and connection details to dbt. It can be overridden on a per-command basis by using the `--profiles-dir` command line argument. | [optional]
**profiles_directory** | Option<**String**> | Optional (relative) path to the profiles directory. Can only be specified if no warehouse_id is specified. If no warehouse_id is specified and this folder is unset, the root directory is used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


