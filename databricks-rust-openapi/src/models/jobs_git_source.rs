use schemars::JsonSchema;
/*
 * Databricks Accounts and Workspace REST API on ALL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JobsGitSource : An optional specification for a remote Git repository containing the source code used by tasks. Version-controlled source code is supported by notebook, dbt, Python script, and SQL File tasks.  If `git_source` is set, these tasks retrieve the file from the remote repository by default. However, this behavior can be overridden by setting `source` to `WORKSPACE` on the task.  Note: dbt and SQL File tasks support only version-controlled sources. If dbt or SQL File tasks are used, `git_source` must be defined on the job.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsGitSource {
    /// Name of the branch to be checked out and used by this job. This field cannot be specified in conjunction with git_tag or git_commit.
    #[serde(rename = "git_branch", skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    /// Commit to be checked out and used by this job. This field cannot be specified in conjunction with git_branch or git_tag.
    #[serde(rename = "git_commit", skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    /// Unique identifier of the service used to host the Git repository. The value is case insensitive.
    #[serde(rename = "git_provider")]
    pub git_provider: crate::models::JobsGitProvider,
    #[serde(rename = "git_snapshot", skip_serializing_if = "Option::is_none")]
    pub git_snapshot: Option<Box<crate::models::JobsGitSnapshot>>,
    /// Name of the tag to be checked out and used by this job. This field cannot be specified in conjunction with git_branch or git_commit.
    #[serde(rename = "git_tag", skip_serializing_if = "Option::is_none")]
    pub git_tag: Option<String>,
    /// URL of the repository to be cloned by this job.
    #[serde(rename = "git_url")]
    pub git_url: String,
    /// The source of the job specification in the remote repository when the job is source controlled.
    #[serde(rename = "job_source", skip_serializing_if = "Option::is_none")]
    pub job_source: Option<Box<crate::models::JobsJobSource>>,
}

impl JobsGitSource {
    /// An optional specification for a remote Git repository containing the source code used by tasks. Version-controlled source code is supported by notebook, dbt, Python script, and SQL File tasks.  If `git_source` is set, these tasks retrieve the file from the remote repository by default. However, this behavior can be overridden by setting `source` to `WORKSPACE` on the task.  Note: dbt and SQL File tasks support only version-controlled sources. If dbt or SQL File tasks are used, `git_source` must be defined on the job.
    pub fn new(git_provider: crate::models::JobsGitProvider, git_url: String) -> JobsGitSource {
        JobsGitSource {
            git_branch: None,
            git_commit: None,
            git_provider,
            git_snapshot: None,
            git_tag: None,
            git_url,
            job_source: None,
        }
    }
}


