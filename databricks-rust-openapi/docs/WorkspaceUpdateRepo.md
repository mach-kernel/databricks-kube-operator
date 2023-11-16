# WorkspaceUpdateRepo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**branch** | Option<**String**> | Branch that the local version of the repo is checked out to. | [optional]
**sparse_checkout** | Option<[**crate::models::WorkspaceSparseCheckoutUpdate**](WorkspaceSparseCheckoutUpdate.md)> | If specified, update the sparse checkout settings. The update will fail if sparse checkout is not enabled for the repo. | [optional]
**tag** | Option<**String**> | Tag that the local version of the repo is checked out to. Updating the repo to a tag puts the repo in a detached HEAD state. Before committing new changes, you must update the repo to a branch instead of the detached HEAD. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


