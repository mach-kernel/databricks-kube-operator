# ComputePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creator_user_name** | Option<**String**> | Creator user name. The field won't be included in the response if the user has already been deleted. | [optional]
**is_default** | Option<**bool**> | If true, policy is a default policy created and managed by <Databricks>. Default policies cannot be deleted, and their policy families cannot be changed. | [optional]
**max_clusters_per_user** | Option<**i64**> | Max number of clusters per user that can be active using this policy. If not present, there is no max limit. | [optional]
**policy_id** | Option<**String**> | Canonical unique identifier for the Cluster Policy. | [optional]
**name** | Option<**String**> | Cluster Policy name requested by the user. This has to be unique. Length must be between 1 and 100 characters. | [optional]
**definition** | Option<**String**> | Policy definition document expressed in Databricks Cluster Policy Definition Language. | [optional]
**policy_family_definition_overrides** | Option<**String**> | Policy definition JSON document expressed in Databricks Policy Definition Language. The JSON document must be passed as a string and cannot be embedded in the requests.  You can use this to customize the policy definition inherited from the policy family. Policy rules specified here are merged into the inherited policy definition.  | [optional]
**policy_family_id** | Option<**String**> | ID of the policy family. | [optional]
**created_at_timestamp** | Option<**i64**> | Creation time. The timestamp (in millisecond) when this Cluster Policy was created. | [optional]
**description** | Option<**String**> | Additional human-readable description of the cluster policy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


