use std::collections::BTreeMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use crate::context::Context;
use databricks_rust_secrets::apis::secret_api;
use databricks_rust_secrets::models::{
    WorkspaceDeleteSecret, WorkspaceGetSecretResponse, WorkspacePutSecret,
};
use futures::{Stream, StreamExt, TryFutureExt, TryStreamExt};
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::ByteString;
use kube::runtime::controller::Action;
use kube::runtime::finalizer::Event;
use kube::runtime::reflector::ObjectRef;
use kube::runtime::{finalizer, watcher, Controller};
use kube::{core::object::HasSpec, CustomResource, CustomResourceExt};
use kube::{Api, ResourceExt};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::DatabricksKubeError;
use crate::traits::rest_config::RestConfig;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "DatabricksSecret",
    derive = "Default",
    namespaced
)]
pub struct DatabricksSecretSpec {
    pub scope: String,
    pub secret_name: String,
}

impl DatabricksSecret {
    pub fn controller(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(ObjectRef<Self>, Action), DatabricksKubeError>> + Send>>
    {
        let kube_api = Api::<Self>::default_namespaced(context.client.clone());

        Controller::new(kube_api, watcher::Config::default())
            .shutdown_on_signal()
            .run(
                Self::reconcile,
                |res, err, _ctx| {
                    log::error!(
                        "Secret Sync failed for {} {} (retrying in 30s):\n{}",
                        Self::api_resource().kind,
                        res.name_unchecked(),
                        err,
                    );
                    Action::requeue(Duration::from_secs(30))
                },
                context.clone(),
            )
            .map_err(|e| DatabricksKubeError::ControllerError(e.to_string()))
            .boxed()
    }

    async fn source_secrets(
        crd: &Arc<Self>,
        context: &Arc<Context>,
    ) -> Result<BTreeMap<String, ByteString>, DatabricksKubeError> {
        let k8s_secrets = Api::<Secret>::default_namespaced(context.client.clone());
        let source_secrets = k8s_secrets
            .get(crd.spec().secret_name.as_str())
            .map_err(|e| DatabricksKubeError::SecretMissingError(e.to_string()))
            .await?
            .data
            .expect("A secret must have data in order to be synced");

        Ok(source_secrets)
    }

    async fn sync_secrets(
        crd: Arc<Self>,
        context: Arc<Context>,
    ) -> Result<Action, DatabricksKubeError> {
        let rest_config = WorkspaceGetSecretResponse::get_rest_config(context.clone())
            .await
            .unwrap();

        for (name, ByteString(bytes)) in Self::source_secrets(&crd, &context).await? {
            log::info!(
                "Syncing {}: {{secrets/{}/{}}})",
                crd.name_unchecked(),
                crd.spec().scope,
                name
            );

            secret_api::put_secret(
                &rest_config,
                Some(WorkspacePutSecret {
                    scope: crd.spec().scope.clone(),
                    string_value: Some(
                        String::from_utf8(bytes)
                            .map_err(|_| {
                                DatabricksKubeError::ResourceUpdateError(
                                    "Unable to encode secret for update".to_string(),
                                )
                            })
                            .unwrap(),
                    ),
                    bytes_value: None,
                    key: name,
                }),
            )
            .await?;
        }

        Ok(Action::requeue(Duration::from_secs(
            context
                .get_operator_config()
                .and_then(|c| c.default_requeue_interval)
                .unwrap_or(300),
        )))
    }

    async fn delete_secrets(
        crd: Arc<Self>,
        context: Arc<Context>,
    ) -> Result<Action, DatabricksKubeError> {
        let rest_config = WorkspaceGetSecretResponse::get_rest_config(context.clone())
            .await
            .unwrap();

        for (name, _) in Self::source_secrets(&crd, &context).await? {
            log::info!(
                "Deleting {}: (scope {}, key {})",
                crd.name_unchecked(),
                crd.spec().scope,
                name
            );

            secret_api::delete_secret(
                &rest_config,
                Some(WorkspaceDeleteSecret {
                    scope: crd.spec().scope.clone(),
                    key: name,
                }),
            )
            .await?;
        }

        Ok(Action::await_change())
    }

    async fn reconcile(
        crd: Arc<Self>,
        context: Arc<Context>,
    ) -> Result<Action, DatabricksKubeError> {
        let kube_api = Api::<DatabricksSecret>::default_namespaced(context.client.clone());

        finalizer(
            &kube_api,
            "databricks-operator/remote_secret",
            crd.clone(),
            |e| async {
                match e {
                    Event::Apply(res) => Self::sync_secrets(res, context.clone()).await,
                    Event::Cleanup(res) => Self::delete_secrets(res, context.clone()).await,
                }
            },
        )
        .map_err(|e| e.into())
        .await
    }
}
