# ComputeDiskSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disk_count** | Option<**i32**> | The number of disks launched for each instance: - This feature is only enabled for supported node types. - Users can choose up to the limit of the disks supported by the node type. - For node types with no OS disk, at least one disk must be specified;   otherwise, cluster creation will fail.  If disks are attached, Databricks will configure Spark to use only the disks for scratch storage, because heterogenously sized scratch devices can lead to inefficient disk utilization. If no disks are attached, Databricks will configure Spark to use instance store disks.  Note: If disks are specified, then the Spark configuration `Sparklocal.dir` will be overridden.  Disks will be mounted at: - For AWS: `/ebs0`, `/ebs1`, and etc. - For Azure: `/remote_volume0`, `/remote_volume1`, and etc. | [optional][default to 0]
**disk_iops** | Option<**i32**> |  | [optional]
**disk_size** | Option<**i32**> | The size of each disk (in GiB) launched for each instance. Values must fall into the supported range for a particular instance type.  For AWS: - General Purpose SSD: 100 - 4096 GiB - Throughput Optimized HDD: 500 - 4096 GiB  For Azure: - Premium LRS (SSD): 1 - 1023 GiB - Standard LRS (HDD): 1- 1023 GiB | [optional]
**disk_throughput** | Option<**i32**> |  | [optional]
**disk_type** | Option<[**crate::models::ComputeDiskType**](ComputeDiskType.md)> | The type of disks that will be launched with this cluster. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


