# ComputeGlobalInitScriptUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Specifies whether the script is enabled. The script runs only if enabled. | [optional]
**name** | **String** | The name of the script | 
**position** | Option<**i32**> | The position of a script, where 0 represents the first script to run, 1 is the second script to run, in ascending order. To move the script to run first, set its position to 0.  To move the script to the end, set its position to any value greater or equal to the position of the last script. Example, three existing scripts with positions 0, 1, and 2. Any position value of 2 or greater puts the script in the last position (2).  If an explicit position value conflicts with an existing script, your request succeeds, but the original script at that position and all later scripts have their positions incremented by 1.  | [optional]
**script** | **String** | The Base64-encoded content of the script. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


