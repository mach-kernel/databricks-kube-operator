# ComputeGcpAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**crate::models::ComputeGcpAvailability**](ComputeGcpAvailability.md)> |  | [optional]
**boot_disk_size** | Option<**i32**> | boot disk size in GB | [optional]
**google_service_account** | Option<**String**> | If provided, the cluster will impersonate the google service account when accessing gcloud services (like GCS). The google service account must have previously been added to the Databricks environment by an account administrator. | [optional]
**local_ssd_count** | Option<**i32**> | If provided, each node (workers and driver) in the cluster will have this number of local SSDs attached. Each local SSD is 375GB in size. Refer to [GCP documentation](https://Cloudgoogle.com/compute/docs/disks/local-ssd#choose_number_local_ssds) for the supported number of local SSDs for each instance type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


