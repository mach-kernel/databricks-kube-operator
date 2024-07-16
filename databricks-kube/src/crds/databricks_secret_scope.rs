use std::pin::Pin;
use std::{sync::Arc, time::SystemTime};

use crate::context::Context;
use async_stream::{stream, try_stream};
use databricks_rust_secrets::apis::secret_api;
use databricks_rust_secrets::models::{
    WorkspaceCreateScope, WorkspaceDeleteScope, WorkspaceListScopesResponse, WorkspaceSecretScope,
};
use futures::{future, Stream, StreamExt, TryStreamExt};
use kube::{core::object::HasSpec, CustomResource};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::traits::rest_config::RestConfig;
use crate::{error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource};

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "DatabricksSecretScope",
    derive = "Default",
    namespaced
)]
pub struct DatabricksSecretScopeSpec {
    pub scope: WorkspaceSecretScope,
    pub initial_manage_principal: Option<String>,
}

// API -> CRD
impl From<WorkspaceSecretScope> for DatabricksSecretScope {
    fn from(scope: WorkspaceSecretScope) -> Self {
        let k8s_name = scope.name.clone().unwrap_or(format!(
            "noname-{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        Self::new(
            &k8s_name,
            DatabricksSecretScopeSpec {
                scope,
                initial_manage_principal: None,
            },
        )
    }
}

// CRD -> API
impl From<DatabricksSecretScope> for WorkspaceSecretScope {
    fn from(value: DatabricksSecretScope) -> Self {
        value.spec().scope.clone()
    }
}

impl RemoteAPIResource<WorkspaceSecretScope> for DatabricksSecretScope {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<WorkspaceSecretScope, DatabricksKubeError>> + Send>> {
        try_stream! {
            let config = WorkspaceSecretScope::get_rest_config(context.clone()).await.unwrap();

            if let WorkspaceListScopesResponse {
                scopes
            } = secret_api::list_scopes(&config).await? {
                if let Some(scopes) = scopes {
                    for scope in scopes {
                        yield scope;
                    }
                }
            }
        }
        .boxed()
    }

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<WorkspaceSecretScope, DatabricksKubeError>> + Send>> {
        let scope_name = self.spec().scope.name.clone().unwrap();

        stream! {
            let remote = Self::remote_list_all(context)
                .try_filter(move |rs| future::ready(rs.clone().name.map(|rsn| rsn == scope_name.clone()).unwrap_or(false)))
                .next()
                .await;

            yield remote.unwrap_or(Err(DatabricksKubeError::IDUnsetError))
        }
        .boxed()
    }

    fn remote_create(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>> {
        let scope = self.spec().scope.clone();

        try_stream! {
            let config = WorkspaceSecretScope::get_rest_config(context.clone()).await.unwrap();

            // Endpoint has no response value
            secret_api::create_scope(&config, Some(WorkspaceCreateScope {
                scope: scope.name.unwrap(),
                initial_manage_principal: self.spec().initial_manage_principal.clone(),
                ..Default::default()
            })).await?;

            yield self.clone()
        }
        .boxed()
    }

    fn remote_update(
        &self,
        _: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>> {
        log::warn!("No change made! Secret scope resources are not updatable. Delete and recreate your resource.");
        try_stream! { yield self.clone() }.boxed()
    }

    fn remote_delete(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        let scope = self.spec().scope.clone();

        try_stream! {
            let config = WorkspaceSecretScope::get_rest_config(context.clone()).await.unwrap();
            secret_api::delete_scope(
                &config,
                Some(WorkspaceDeleteScope { scope: scope.name.unwrap() })
            ).await?;

            yield ();
        }
        .boxed()
    }
}
