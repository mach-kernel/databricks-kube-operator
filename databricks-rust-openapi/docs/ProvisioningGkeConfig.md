# ProvisioningGkeConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connectivity_type** | Option<**String**> | Specifies the network connectivity types for the GKE nodes and the GKE master network.  Set to `PRIVATE_NODE_PUBLIC_MASTER` for a private GKE cluster for the workspace. The GKE nodes will not have public IPs.  Set to `PUBLIC_NODE_PUBLIC_MASTER` for a public GKE cluster. The nodes of a public GKE cluster have public IP addresses.  | [optional]
**master_ip_range** | Option<**String**> | The IP range from which to allocate GKE cluster master resources. This field will be ignored if GKE private cluster is not enabled.  It must be exactly as big as `/28`. | [optional][default to 10.3.0.0/28]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


