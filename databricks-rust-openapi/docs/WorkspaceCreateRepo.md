# WorkspaceCreateRepo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | Option<**String**> | Desired path for the repo in the workspace. Must be in the format /Repos/{folder}/{repo-name}. | [optional]
**provider** | **String** | Git provider. This field is case-insensitive. The available Git providers are gitHub, bitbucketCloud, gitLab, azureDevOpsServices, gitHubEnterprise, bitbucketServer, gitLabEnterpriseEdition and awsCodeCommit. | 
**sparse_checkout** | Option<[**crate::models::WorkspaceSparseCheckout**](WorkspaceSparseCheckout.md)> | If specified, the repo will be created with sparse checkout enabled. You cannot enable/disable sparse checkout after the repo is created. | [optional]
**url** | **String** | URL of the Git repository to be linked. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


