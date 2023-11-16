use std::sync::Arc;
use std::{pin::Pin, time::SystemTime};

use crate::context::Context;
use crate::traits::rest_config::RestConfig;
use crate::{error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource};

use async_stream::try_stream;
use databricks_rust_openapi::apis::repos_api;
use databricks_rust_openapi::models::{WorkspaceRepoInfo, WorkspaceListReposResponse, WorkspaceCreateRepo, WorkspaceUpdateRepo, WorkspaceSparseCheckoutUpdate};
use futures::{Stream, StreamExt};
use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{core::object::HasSpec, CustomResource};

use schemars::JsonSchema;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "Repo",
    derive = "Default",
    namespaced
)]
pub struct RepoSpec {
    pub repository: WorkspaceRepoInfo,
}

/// API -> CRD
impl From<WorkspaceRepoInfo> for Repo {
    fn from(repository: WorkspaceRepoInfo) -> Self {
        let repo_name = if let Some(cid) = &repository.id {
            cid.to_string()
        } else {
            format!(
                "noname-{}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            )
        };

        Self::new(&repo_name, RepoSpec { repository })
    }
}

/// CRD -> API
impl From<Repo> for WorkspaceRepoInfo {
    fn from(value: Repo) -> Self {
        value.spec().repository.clone()
    }
}

impl RemoteAPIResource<WorkspaceRepoInfo> for Repo {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<WorkspaceRepoInfo, DatabricksKubeError>> + Send>> {
        try_stream! {
            let config = WorkspaceRepoInfo::get_rest_config(context.clone()).await.unwrap();
            let mut next_page: Option<String> = None;

            while let WorkspaceListReposResponse {
                repos,
                next_page_token,
            } = repos_api::reposlist(&config, None, next_page.map(|s| s.to_owned()).as_ref().map(|x| &**x)).await? {
                if let Some(repos) = repos {
                    for repo in repos {
                        yield repo;
                    }
                }

                if next_page_token.is_some() {
                    next_page = next_page_token;
                } else {
                    break;
                }
            }
        }
        .boxed()
    }

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<WorkspaceRepoInfo, DatabricksKubeError>> + Send>> {
        let repository_id = self
            .spec()
            .repository
            .id
            .ok_or(DatabricksKubeError::APIError(
                "Remote resource cannot exist".to_string(),
            ));

        try_stream! {
            let config = WorkspaceRepoInfo::get_rest_config(context.clone()).await.unwrap();
            let res = repos_api::reposget(&config, repository_id?).await?;
            yield res
        }
        .boxed()
    }

    fn remote_create(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let repository = self.spec().repository.clone();

        try_stream! {
            let config = WorkspaceRepoInfo::get_rest_config(context.clone()).await.unwrap();

            let new_repo = repos_api::reposcreate(
                &config,
                WorkspaceCreateRepo {
                    url: repository.url.unwrap(),
                    provider: repository.provider.unwrap(),
                    path: repository.path,
                    sparse_checkout: repository.sparse_checkout
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.repository = new_repo;
            yield with_response;
        }
        .boxed()
    }

    fn remote_update(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let repository = self.spec().repository.clone();

        try_stream! {
            let config = WorkspaceRepoInfo::get_rest_config(context.clone()).await.unwrap();
            // TODO
            let _sparse_checkout = repository.sparse_checkout.map(|s| WorkspaceSparseCheckoutUpdate {
                patterns: s.patterns
            });

            repos_api::reposupdate(
                &config,
                repository.id.expect("Need Repo ID for update"),
                WorkspaceUpdateRepo {
                    branch: repository.branch,
                    tag: None,
                    // TODO
                    sparse_checkout: None
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.repository = self.remote_get(context.clone()).next().await.unwrap()?;
            yield with_response;
        }
        .boxed()
    }

    fn remote_delete(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        let repository_id = self.spec().repository.id;

        try_stream! {
            let config = WorkspaceRepoInfo::get_rest_config(context.clone()).await.unwrap();
            repos_api::reposdelete(
                &config,
                repository_id.expect("Need Repo ID for delete")
            ).await?;

            yield ()
        }
        .boxed()
    }
}
