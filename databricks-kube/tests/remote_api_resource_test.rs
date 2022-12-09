mod common;

use std::{collections::BTreeMap, pin::Pin, sync::Arc, time::Duration};

use common::fake_resource::{FakeAPIResource, FakeResource, FakeResourceSpec};
use common::mock_k8s::{
    mock_fake_resource_created, mock_fake_resource_deleted, mock_fake_resource_updated_kube,
    mock_list_fake_resource,
};

use databricks_kube::{
    context::Context, error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource,
};

use async_stream::try_stream;
use flurry::HashMap;
use futures::{Future, FutureExt, Stream, StreamExt};
use hyper::Body;

use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use k8s_openapi::chrono::Utc;
use k8s_openapi::http::{Request, Response};
use kube::runtime::controller::Action;
use kube::runtime::reflector::ObjectRef;
use kube::{
    core::object::HasSpec,
    runtime::reflector::{self, store::Writer, Store},
    Client, Resource,
};
use lazy_static::lazy_static;
use tokio::time::{sleep, timeout};
use tower_test::mock::Handle;
use tower_test::mock::{self};

use crate::context::{DatabricksAPISecret, OperatorConfiguration};

/*
 * A basic integration test for the RemoteAPIResource trait against a FakeResource CRD.
 *
 * This implementation treats TEST_STORE as the remote state store, which we can then
 * assert against for test cases.
 */
lazy_static! {
    static ref TEST_STORE: HashMap<i64, FakeAPIResource> = HashMap::new();
}

impl RemoteAPIResource<FakeAPIResource> for FakeResource {
    fn remote_list_all(
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeAPIResource, DatabricksKubeError>> + Send>> {
        let resources = TEST_STORE.pin();
        let resources: Vec<FakeAPIResource> = resources.values().map(Clone::clone).collect();

        try_stream! {
            for resource in resources {
                yield resource.clone();
            }
        }
        .boxed()
    }

    fn every_reconcile_owned(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn futures::Future<Output = Result<(), DatabricksKubeError>> + Send>> {
        TEST_STORE.pin().insert(
            -8675309,
            FakeAPIResource {
                id: -8675309,
                description: Some("ask for jenny".to_string()),
            },
        );
        async { Ok(()) }.boxed()
    }

    fn remote_get(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeAPIResource, DatabricksKubeError>> + Send>> {
        let found = TEST_STORE
            .pin()
            .get(&self.spec().api_resource.id)
            .map(Clone::clone);

        try_stream! {
            yield found.ok_or_else(|| DatabricksKubeError::APIError("Not found".to_string()))?;
        }
        .boxed()
    }

    fn remote_create(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeResource, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let api_resource = self.spec().api_resource.clone();
        try_stream! {
            TEST_STORE.pin().insert(api_resource.id, api_resource.clone());
            yield self.clone();
        }
        .boxed()
    }

    fn remote_update(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeResource, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        try_stream! {
            TEST_STORE.pin().insert(self.spec().api_resource.id, self.spec().api_resource.clone());
            yield self.clone()
        }
        .boxed()
    }

    fn remote_delete(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        try_stream! {
            TEST_STORE.pin().remove_entry(&self.spec().api_resource.id);
            yield ()
        }
        .boxed()
    }
}

/// Convenience wrapper: provide a kube mock future,
/// and a function that consumes the controller for assertions
async fn with_mocked_kube_server_and_controller<A, B>(
    f_mock: impl Fn(Handle<Request<Body>, Response<Body>>) -> A + Send + Sync + 'static,
    f_assert: impl Fn(
        Pin<
            Box<
                (dyn Stream<Item = Result<(ObjectRef<FakeResource>, Action), DatabricksKubeError>>
                     + std::marker::Send
                     + 'static),
            >,
        >,
    ) -> B,
) where
    A: Future + Send + 'static,
    A::Output: Send + 'static,
    B: Future + Send + 'static,
    B::Output: Send + 'static,
{
    let (mock_service, handle) = mock::pair::<Request<Body>, Response<Body>>();
    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let context = Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    );

    let kube_server = tokio::spawn(async move {
        f_mock(handle).await;
    });

    f_assert(FakeResource::controller(context.clone())).await;

    kube_server.abort();
    TEST_STORE.pin().clear();
}

/// When the resource is created in Kubernetes
#[tokio::test]
async fn test_resource_created() {
    let created_api_resource = FakeAPIResource {
        id: 1,
        description: None,
    };

    let mut created_resource: FakeResource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: created_api_resource.clone(),
        },
    );

    created_resource.meta_mut().resource_version = Some("1".to_string());
    created_resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    // Bind the finalizer to avoid having to mock the PATCH request from the API
    created_resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let cr = created_resource.clone();

            async move {
                loop {
                    mock_fake_resource_created(&mut handle, cr.clone()).await
                }
            }
        },
        move |mut controller| {
            let created = created_api_resource.clone();

            async move {
                let reconciled = controller.next().await;
                assert!(reconciled.unwrap().is_ok());
                assert_eq!(TEST_STORE.pin().get(&1).unwrap().clone(), created);
            }
        },
    )
    .await;
}

/// When an owned resource is updated in Kubernetes
#[tokio::test]
async fn test_resource_kube_update_operator_owned() {
    let mut resource: FakeResource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );

    TEST_STORE
        .pin()
        .insert(1, resource.spec.api_resource.clone());

    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    let updated_api_resource = FakeAPIResource {
        id: 1,
        description: Some("foobar".to_string()),
    };

    let mut updated_resource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: updated_api_resource.clone(),
        },
    );

    updated_resource.meta_mut().resource_version = Some("2".to_string());
    updated_resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let r = resource.clone();
            let ur = updated_resource.clone();

            async move {
                loop {
                    mock_fake_resource_updated_kube(
                        &mut handle,
                        r.clone(),
                        ur.clone(),
                        ur.clone().spec().api_resource.clone(),
                        Some("MODIFIED".to_string()),
                    )
                    .await;
                }
            }
        },
        move |mut controller| {
            let updated = updated_api_resource.clone();

            async move {
                // It reconciled successfully and the resources are in sync
                let reconciled = controller.next().await;
                assert!(reconciled.unwrap().is_ok());
                assert_eq!(TEST_STORE.pin().get(&1).unwrap().clone(), updated,);

                // every_reconcile() was triggered
                assert!(TEST_STORE.pin().contains_key(&-8675309));
            }
        },
    )
    .await;
}

/// When an API owned resource is updated in Kubernetes
#[tokio::test]
async fn test_resource_kube_update_api_owned() {
    let api_resource = FakeAPIResource {
        id: 1,
        description: None,
    };
    TEST_STORE.pin().insert(1, api_resource.clone());

    let mut resource: FakeResource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: api_resource.clone(),
        },
    );
    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert("databricks-operator/owner".to_string(), "api".to_string());
        annots
    });
    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    let updated_api_resource = FakeAPIResource {
        id: 1,
        description: Some("foobar".to_string()),
    };

    let mut updated_resource = resource.clone();
    updated_resource.spec.api_resource = updated_api_resource.clone();
    updated_resource.meta_mut().resource_version = Some("2".to_string());
    updated_resource.meta_mut().finalizers = resource.meta().finalizers.clone();

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let r = resource.clone();
            let ur = updated_resource.clone();
            let ar = api_resource.clone();

            async move {
                loop {
                    mock_fake_resource_updated_kube(
                        &mut handle,
                        r.clone(),
                        ur.clone(),
                        ar.clone(),
                        Some("MODIFIED".to_string()),
                    )
                    .await;
                }
            }
        },
        |mut controller| async move {
            // It reconciled successfully
            let reconciled = controller.next().await;
            assert!(reconciled.unwrap().is_ok());

            // every_reconcile() was NOT triggered as the resource is not owned
            assert!(!TEST_STORE.pin().contains_key(&-8675309));

            // The object is the original API object
            assert_eq!(
                TEST_STORE.pin().get(&1).unwrap().clone(),
                FakeAPIResource {
                    id: 1,
                    description: None
                },
            );
        },
    )
    .await;
}

/// When the API resource is updated for an owned resource
#[tokio::test]
async fn test_resource_api_update_operator_owned() {
    // Begin with a CRD owned by the operator
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 42,
                description: None,
            },
        },
    );

    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    // Remote has a different value for "description"
    let updated_resource = FakeAPIResource {
        description: Some("hello".to_string()),
        ..resource.spec().api_resource
    };
    TEST_STORE.pin().insert(42, updated_resource.clone());

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let r = resource.clone();

            async move {
                loop {
                    mock_list_fake_resource(
                        &mut handle,
                        r.clone(),
                        // assertion made during PUT call
                        Some(r.spec().api_resource.clone()),
                        None,
                    )
                    .await;
                }
            }
        },
        |mut controller| async move {
            // It reconciled successfully
            let reconciled = controller.next().await;
            assert!(reconciled.unwrap().is_ok());

            // every_reconcile() was triggered
            assert!(TEST_STORE.pin().contains_key(&-8675309));
        },
    )
    .await;
}

/// When the API resource is updated for an API owned resource
#[tokio::test]
async fn test_resource_api_update_api_owned() {
    // Begin with a CRD owned by the operator
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 42,
                description: None,
            },
        },
    );

    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert("databricks-operator/owner".to_string(), "api".to_string());
        annots
    });
    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    // Remote has a different value for "description"
    let updated_resource = FakeAPIResource {
        description: Some("hello".to_string()),
        ..resource.spec().api_resource
    };
    TEST_STORE.pin().insert(42, updated_resource.clone());

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let r = resource.clone();
            let ur = updated_resource.clone();

            async move {
                loop {
                    mock_list_fake_resource(
                        &mut handle,
                        r.clone(),
                        // assertion made during PUT call
                        Some(ur.clone()),
                        None,
                    )
                    .await;
                }
            }
        },
        |mut controller| async move {
            // It reconciled successfully
            let reconciled = controller.next().await;
            assert!(reconciled.unwrap().is_ok());

            // every_reconcile() was NOT triggered as the resource is not owned
            assert!(!TEST_STORE.pin().contains_key(&-8675309));
        },
    )
    .await;
}

/// When an owned Kubernetes resource matches the remote API
#[tokio::test]
async fn test_resource_in_sync() {
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );
    resource.meta_mut().namespace = Some("default".to_string());
    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });
    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    TEST_STORE
        .pin()
        .insert(1, resource.spec().api_resource.clone());

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let r = resource.clone();

            async move {
                loop {
                    mock_list_fake_resource(
                        &mut handle,
                        r.clone(),
                        Some(r.spec().api_resource.clone()),
                        None,
                    )
                    .await;
                }
            }
        },
        |mut controller| async move {
            // It reconciled successfully and the resources are in sync
            let reconciled = controller.next().await;
            assert!(reconciled.unwrap().is_ok());

            // every_reconcile() was triggered
            assert!(TEST_STORE.pin().contains_key(&-8675309));
        },
    )
    .await;
}

// When an owned Kubernetes resource is deleted
#[tokio::test]
async fn test_kube_delete_operator_owned() {
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );

    resource.meta_mut().namespace = Some("default".to_string());
    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    // Mark the resource as deleted
    resource.meta_mut().deletion_timestamp = Some(Time(Utc::now()));

    TEST_STORE
        .pin()
        .insert(1, resource.spec().api_resource.clone());

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let serve_me = resource.clone();

            async move {
                loop {
                    mock_fake_resource_deleted(&mut handle, serve_me.clone()).await;
                }
            }
        },
        |mut controller| async move {
            // It reconciled successfully and the resources are in sync
            let reconciled = controller.next().await;
            assert!(reconciled.unwrap().is_ok());

            // The resource was removed from the remote API
            assert!(TEST_STORE.pin().get(&1).is_none());
        },
    )
    .await;

    let default_operator = OperatorConfiguration::default();

    let poll_interval = default_operator.default_poll_interval.parse::<u64>().unwrap();
    let default_timeout_seconds = default_operator.default_timeout_seconds.parse::<u64>().unwrap();

    // We don't yield the watch stream in our task, so we have to wait
    // for the effect to happen
    let poll_store = async {
        while let Some(_) = TEST_STORE.pin().get(&1) {
            sleep(Duration::from_millis(poll_interval)).await;
        }
    };
    timeout(Duration::from_secs(default_timeout_seconds), poll_store).await.unwrap();
}

// When Kubernetes resource is deleted, but owned by remote API
#[tokio::test]
async fn test_kube_delete_api_owned() {
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );

    resource.meta_mut().namespace = Some("default".to_string());
    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert("databricks-operator/owner".to_string(), "api".to_string());
        annots
    });
    // Bind the finalizer to avoid having to mock the PATCH request from the API
    resource.meta_mut().finalizers =
        Some(vec!["databricks-operator/remote_api_resource".to_owned()]);

    // Mark the resource as deleted
    resource.meta_mut().deletion_timestamp = Some(Time(Utc::now()));

    TEST_STORE
        .pin()
        .insert(1, resource.spec().api_resource.clone());

    with_mocked_kube_server_and_controller(
        move |mut handle| {
            let serve_me = resource.clone();

            async move {
                loop {
                    mock_fake_resource_deleted(&mut handle, serve_me.clone()).await;
                }
            }
        },
        |mut controller| async move {
            // It reconciled successfully and the resources are in sync
            let reconciled = controller.next().await;
            assert!(reconciled.unwrap().is_ok());

            // The resource was NOT removed from the remote API
            assert!(TEST_STORE.pin().get(&1).is_some());
        },
    )
    .await;
}
