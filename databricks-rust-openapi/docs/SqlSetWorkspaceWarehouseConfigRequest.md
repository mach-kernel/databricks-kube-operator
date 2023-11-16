# SqlSetWorkspaceWarehouseConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data_access_config** | Option<[**Vec<crate::models::SqlEndpointConfPair>**](SqlEndpointConfPair.md)> |  | [optional]
**security_policy** | Option<**String**> | Security policy for warehouses | [optional]
**global_param** | Option<[**crate::models::SqlRepeatedEndpointConfPairs**](SqlRepeatedEndpointConfPairs.md)> | Deprecated: Use sql_configuration_parameters | [optional]
**enabled_warehouse_types** | Option<[**Vec<crate::models::SqlWarehouseTypePair>**](SqlWarehouseTypePair.md)> |  | [optional]
**instance_profile_arn** | Option<**String**> | AWS Only: Instance profile used to pass IAM role to the cluster | [optional]
**google_service_account** | Option<**String**> | GCP only: Google Service Account used to pass to cluster to access Google Cloud Storage | [optional]
**sql_configuration_parameters** | Option<[**crate::models::SqlRepeatedEndpointConfPairs**](SqlRepeatedEndpointConfPairs.md)> | SQL configuration parameters | [optional]
**config_param** | Option<[**crate::models::SqlRepeatedEndpointConfPairs**](SqlRepeatedEndpointConfPairs.md)> | Deprecated: Use sql_configuration_parameters | [optional]
**channel** | Option<[**crate::models::SqlChannel**](SqlChannel.md)> | Optional: Channel selection details | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


