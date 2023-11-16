# JobsGitSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**git_branch** | Option<**String**> | Name of the branch to be checked out and used by this job. This field cannot be specified in conjunction with git_tag or git_commit. | [optional]
**git_commit** | Option<**String**> | Commit to be checked out and used by this job. This field cannot be specified in conjunction with git_branch or git_tag. | [optional]
**git_provider** | [**crate::models::JobsGitProvider**](JobsGitProvider.md) | Unique identifier of the service used to host the Git repository. The value is case insensitive. | 
**git_snapshot** | Option<[**crate::models::JobsGitSnapshot**](JobsGitSnapshot.md)> |  | [optional]
**git_tag** | Option<**String**> | Name of the tag to be checked out and used by this job. This field cannot be specified in conjunction with git_branch or git_commit. | [optional]
**git_url** | **String** | URL of the repository to be cloned by this job. | 
**job_source** | Option<[**crate::models::JobsJobSource**](JobsJobSource.md)> | The source of the job specification in the remote repository when the job is source controlled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


