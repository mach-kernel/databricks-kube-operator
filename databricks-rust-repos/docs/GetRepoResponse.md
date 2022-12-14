# GetRepoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | ID of the repo object in the workspace. | [optional]
**url** | Option<**String**> | URL of the Git repository to be linked. | [optional]
**provider** | Option<**String**> | Git provider. This field is case-insensitive. The available Git providers are gitHub, bitbucketCloud, gitLab, azureDevOpsServices, gitHubEnterprise, bitbucketServer, gitLabEnterpriseEdition and awsCodeCommit. | [optional]
**path** | Option<**String**> | Desired path for the repo in the workspace. Must be in the format /Repos/{folder}/{repo-name}. | [optional]
**branch** | Option<**String**> | Branch that the local version of the repo is checked out to. | [optional]
**head_commit_id** | Option<**String**> | SHA-1 hash representing the commit ID of the current HEAD of the repo. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


