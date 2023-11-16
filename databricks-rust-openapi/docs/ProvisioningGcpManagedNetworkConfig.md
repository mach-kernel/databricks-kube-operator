# ProvisioningGcpManagedNetworkConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gke_cluster_pod_ip_range** | Option<**String**> | The IP range from which to allocate GKE cluster pods. No bigger than `/9` and no smaller than `/21`. | [optional][default to 10.1.0.0/16]
**gke_cluster_service_ip_range** | Option<**String**> | The IP range from which to allocate GKE cluster services. No bigger than `/16` and no smaller than `/27`. | [optional][default to 10.2.0.0/20]
**subnet_cidr** | Option<**String**> | The IP range from which to allocate GKE cluster nodes. No bigger than `/9` and no smaller than `/29`. | [optional][default to 10.0.0.0/16]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


