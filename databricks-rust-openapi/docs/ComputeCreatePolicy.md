# ComputeCreatePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**definition** | Option<**String**> | Policy definition document expressed in Databricks Cluster Policy Definition Language. | [optional]
**description** | Option<**String**> | Additional human-readable description of the cluster policy. | [optional]
**max_clusters_per_user** | Option<**i64**> | Max number of clusters per user that can be active using this policy. If not present, there is no max limit. | [optional]
**name** | **String** | Cluster Policy name requested by the user. This has to be unique. Length must be between 1 and 100 characters. | 
**policy_family_definition_overrides** | Option<**String**> | Policy definition JSON document expressed in Databricks Policy Definition Language. The JSON document must be passed as a string and cannot be embedded in the requests.  You can use this to customize the policy definition inherited from the policy family. Policy rules specified here are merged into the inherited policy definition.  | [optional]
**policy_family_id** | Option<**String**> | ID of the policy family. The cluster policy's policy definition inherits the policy family's policy definition.  Cannot be used with `definition`. Use `policy_family_definition_overrides` instead to customize the policy definition.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


