# SharingSharedDataObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shared_as** | Option<**String**> | A user-provided new name for the data object within the share. If this new name is not provided, the object's original name will be used as the `shared_as` name. The `shared_as` name must be unique within a share. For tables, the new name must follow the format of `<schema>.<table>`. | [optional]
**added_at** | Option<**i64**> | The time when this data object is added to the share, in epoch milliseconds. | [optional][readonly]
**history_data_sharing_status** | Option<**String**> | Whether to enable or disable sharing of data history. If not specified, the default is **DISABLED**. | [optional]
**data_object_type** | Option<**String**> | The type of the data object. | [optional][readonly]
**start_version** | Option<**i64**> | The start version associated with the object. This allows data providers to control the lowest object version that is accessible by clients. If specified, clients can query snapshots or changes for versions >= start_version. If not specified, clients can only query starting from the version of the object at the time it was added to the share.  NOTE: The start_version should be <= the `current` version of the object.  | [optional]
**added_by** | Option<**String**> | Username of the sharer. | [optional][readonly]
**name** | **String** | A fully qualified name that uniquely identifies a data object.  For example, a table's fully qualified name is in the format of `<catalog>.<schema>.<table>`.  | 
**status** | Option<**String**> | One of: **ACTIVE**, **PERMISSION_DENIED**. | [optional]
**cdf_enabled** | Option<**bool**> | Whether to enable cdf or indicate if cdf is enabled on the shared object. | [optional]
**comment** | Option<**String**> | A user-provided comment when adding the data object to the share. [Update:OPT] | [optional]
**partitions** | Option<[**Vec<crate::models::SharingPartition>**](SharingPartition.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


