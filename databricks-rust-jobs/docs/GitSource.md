# GitSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**git_url** | Option<**String**> | URL of the repository to be cloned by this job. The maximum length is 300 characters. | [optional]
**git_provider** | Option<**String**> | Unique identifier of the service used to host the Git repository. The value is case insensitive. | [optional]
**git_branch** | Option<**String**> | Name of the branch to be checked out and used by this job. This field cannot be specified in conjunction with git_tag or git_commit. The maximum length is 255 characters. | [optional]
**git_tag** | Option<**String**> | Name of the tag to be checked out and used by this job. This field cannot be specified in conjunction with git_branch or git_commit. The maximum length is 255 characters. | [optional]
**git_commit** | Option<**String**> | Commit to be checked out and used by this job. This field cannot be specified in conjunction with git_branch or git_tag. The maximum length is 64 characters. | [optional]
**git_snapshot** | Option<[**crate::models::GitSnapshot**](GitSnapshot.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


