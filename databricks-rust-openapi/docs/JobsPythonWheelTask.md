# JobsPythonWheelTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entry_point** | Option<**String**> | Named entry point to use, if it does not exist in the metadata of the package it executes the function from the package directly using `$packageName.$entryPoint()` | [optional]
**named_parameters** | Option<**::std::collections::HashMap<String, String>**> | Command-line parameters passed to Python wheel task in the form of `[\"--name=task\", \"--data=dbfs:/path/to/Datajson\"]`. Leave it empty if `parameters` is not null. | [optional]
**package_name** | Option<**String**> | Name of the package to execute | [optional]
**parameters** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


