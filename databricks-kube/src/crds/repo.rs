use std::sync::Arc;
use std::{pin::Pin, time::SystemTime};

use crate::context::Context;
use crate::traits::rest_config::RestConfig;
use crate::{error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource};

use databricks_rust_repos::{
    apis::default_api,
    models::{CreateRepoRequest, GetRepoResponse as APIRepo, GetReposResponse, UpdateRepoRequest},
};

use async_stream::try_stream;
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
    pub repository: APIRepo,
}

/// API -> CRD
impl From<APIRepo> for Repo {
    fn from(repository: APIRepo) -> Self {
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
impl From<Repo> for APIRepo {
    fn from(value: Repo) -> Self {
        value.spec().repository.clone()
    }
}

impl RemoteAPIResource<APIRepo> for Repo {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<APIRepo, DatabricksKubeError>> + Send>> {
        try_stream! {
            let config = APIRepo::get_rest_config(context.clone()).await.unwrap();
            let mut next_page: Option<String> = None;

            while let GetReposResponse {
                repos,
                next_page_token,
            } = default_api::get_repos(&config, None, next_page.map(|s| s.to_owned()).as_ref().map(|x| &**x)).await? {
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
    ) -> Pin<Box<dyn Stream<Item = Result<APIRepo, DatabricksKubeError>> + Send>> {
        let repository_id = self
            .spec()
            .repository
            .id
            .ok_or(DatabricksKubeError::ControllerError(
                "Cannot fetch remote resource when repository_id is undefined".to_string(),
            ));

        try_stream! {
            let config = APIRepo::get_rest_config(context.clone()).await.unwrap();
            let res = default_api::get_repo(&config, &repository_id?.to_string()).await?;
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
            let config = APIRepo::get_rest_config(context.clone()).await.unwrap();

            let new_repo = default_api::create_repo(
                &config,
                CreateRepoRequest {
                    url: repository.url.unwrap(),
                    provider: repository.provider.unwrap(),
                    path: repository.path,
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
        let repository_id = repository.id.unwrap().to_string();

        try_stream! {
            let config = APIRepo::get_rest_config(context.clone()).await.unwrap();

            let new_repo = default_api::update_repo(
                &config,
                &repository_id,
                UpdateRepoRequest {
                    branch: repository.branch.unwrap(),
                    tag: "todo".to_string(),
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.repository = new_repo;
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
            let config = APIRepo::get_rest_config(context.clone()).await.unwrap();
            default_api::delete_repo(
                &config,
                &repository_id.map(|i| i.to_string()).unwrap()
            ).await?;

            yield ()
        }
        .boxed()
    }
}
