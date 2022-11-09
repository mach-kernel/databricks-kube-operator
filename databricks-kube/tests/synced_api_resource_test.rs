mod common;
use common::fake_resource::{FakeAPIResource, FakeResource, FakeResourceSpec};
use databricks_kube::traits::synced_api_resource::ingest_task;
use k8s_openapi::api::core::v1::{ConfigMap, Secret};

use std::sync::Arc;
use std::{collections::BTreeMap, pin::Pin};

use databricks_kube::{
    context::Context,
    error::DatabricksKubeError,
    traits::synced_api_resource::{spawn_delete_watcher, SyncedAPIResource},
};

use async_stream::try_stream;
use futures::{Stream, StreamExt};
use kube::{
    core::object::HasSpec,
    runtime::reflector::{self, store::Writer, Store},
    Client, Resource,
};

use flurry::HashMap;
use lazy_static::lazy_static;

use hyper::Body;
use k8s_openapi::http::{Request, Response};
use tower_test::mock;

use common::mock_k8s::{
    mock_fake_resource_created, mock_fake_resource_deleted, mock_fake_resource_updated_kube,
    mock_ingest_resources, mock_list_fake_resource,
};

use std::time::Duration;
use tokio::time::{sleep, timeout};

/*
 * A basic integration test for the SyncedAPIResource trait against a FakeResource CRD.
 *
 * This implementation treats TEST_STORE as the remote state store, which we can then
 * assert against for test cases.
 */
lazy_static! {
    static ref TEST_STORE: HashMap<i64, FakeAPIResource> = HashMap::new();
}

impl SyncedAPIResource<FakeAPIResource, ()> for FakeResource {
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

/// When the resource is created in Kubernetes
#[tokio::test]
async fn test_controller_lifecycle_created() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

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

    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_created(&mut handle, created_resource.clone()).await
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let mut controller = FakeResource::controller(Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    ));

    // It reconciled successfully and the resources are in sync
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());
    assert_eq!(
        TEST_STORE.pin().get(&1).unwrap().clone(),
        created_api_resource
    );

    kube_server.abort();
    TEST_STORE.pin().clear();
}

/// When an owned resource is updated in Kubernetes
#[tokio::test]
async fn test_controller_lifecycle_update_kube_operator_owned() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    let mut resource: FakeResource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );

    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

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

    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_updated_kube(
                &mut handle,
                resource.clone(),
                updated_resource.clone(),
                Some("MODIFIED".to_string()),
            )
            .await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let mut controller = FakeResource::controller(Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    ));

    // It reconciled successfully and the resources are in sync
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());
    assert_eq!(
        TEST_STORE.pin().get(&1).unwrap().clone(),
        updated_api_resource,
    );

    kube_server.abort();
    TEST_STORE.pin().clear();
}

/// When the API resource is updated for an owned resource
#[tokio::test]
async fn test_controller_lifecycle_update_api_operator_owned() {
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

    // Remote has a different value for "description"
    let updated_resource = FakeAPIResource {
        description: Some("hello".to_string()),
        ..resource.spec().api_resource
    };
    TEST_STORE.pin().insert(42, updated_resource.clone());

    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    // Kube has a different version
    let kube_server = tokio::spawn(async move {
        loop {
            mock_list_fake_resource(
                &mut handle,
                resource.clone(),
                // assertion made during PUT call
                resource.spec().api_resource.clone(),
            )
            .await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let mut controller = FakeResource::controller(Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    ));

    // It reconciled successfully
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());

    kube_server.abort();
    TEST_STORE.pin().clear();
}

/// When an owned Kubernetes resource matches the remote API
#[tokio::test]
async fn test_controller_lifecycle_in_sync_operator_owned() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

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

    TEST_STORE
        .pin()
        .insert(1, resource.spec().api_resource.clone());

    let kube_server = tokio::spawn(async move {
        loop {
            mock_list_fake_resource(
                &mut handle,
                resource.clone(),
                resource.spec().api_resource.clone(),
            )
            .await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let context = Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    );

    let mut controller = FakeResource::controller(context.clone());

    // The reconcile spawns a delete watcher since the resource is owned
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());
    assert!(context
        .delete_watchers
        .pin()
        .get("/apis/com.dstancu.test/v1/namespaces/default/fakeresources/foo")
        .is_some());

    kube_server.abort();
    TEST_STORE.pin().clear();
}

// When an owned Kubernetes resource is deleted
#[tokio::test]
async fn test_controller_lifecycle_delete_operator_owned() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

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

    TEST_STORE
        .pin()
        .insert(1, resource.spec().api_resource.clone());

    let serve_me = resource.clone();
    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_deleted(&mut handle, serve_me.clone()).await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");
    let context = Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    );

    let resource = Arc::new(resource);

    // Create watcher and insert it into the context
    let watcher = spawn_delete_watcher(resource.clone(), context.clone()).await;
    context
        .delete_watchers
        .pin()
        .insert(resource.self_url_unchecked(), watcher.into());

    // We don't yield the watch stream in our task, so we have to wait
    // for the effect to happen
    let poll_store = async {
        while let Some(_) = TEST_STORE.pin().get(&1) {
            sleep(Duration::from_millis(250)).await;
        }
    };
    timeout(Duration::from_secs(10), poll_store).await.unwrap();

    // The resource was removed from the remote API
    assert!(TEST_STORE.pin().get(&1).is_none());
    // The watcher for the resource was aborted
    assert!(context
        .delete_watchers
        .pin()
        .get(&resource.self_url_unchecked())
        .is_none());

    kube_server.abort();
    TEST_STORE.pin().clear();
}

// When Kubernetes resource is deleted, but owned by remote API
#[tokio::test]
async fn test_controller_lifecycle_delete_api_owned() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

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

    TEST_STORE
        .pin()
        .insert(1, resource.spec().api_resource.clone());

    let serve_me = resource.clone();
    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_deleted(&mut handle, serve_me.clone()).await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");
    let context = Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    );

    let resource = Arc::new(resource);

    // Create watcher and insert it into the context
    let watcher = spawn_delete_watcher(resource.clone(), context.clone()).await;
    context
        .delete_watchers
        .pin()
        .insert(resource.self_url_unchecked(), watcher.into());

    // The resource was NOT removed from the remote API
    assert!(TEST_STORE.pin().get(&1).is_some());

    // We don't yield the watch stream in our task, so we have to wait
    // for the effect to happen
    let poll_store = async {
        while let Some(_) = context
            .delete_watchers
            .pin()
            .get(&resource.self_url_unchecked())
        {
            sleep(Duration::from_micros(250)).await;
        }
    };
    timeout(Duration::from_secs(10), poll_store).await.unwrap();

    // The watcher for the resource was still aborted
    assert!(context
        .delete_watchers
        .pin()
        .get(&resource.self_url_unchecked())
        .is_none());

    kube_server.abort();
    TEST_STORE.pin().clear();
}

// Test that the ingest task makes the correct resources
#[allow(unused_must_use)]
#[tokio::test]
async fn test_ingest_task() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    // Make some "remote" resources
    let remote_resources: Vec<FakeAPIResource> = (0..5)
        .map(|id| FakeAPIResource {
            id,
            description: None,
        })
        .collect();
    for res in &remote_resources {
        TEST_STORE.pin().insert(res.id, res.clone());
    }

    // The mock server has assertions for the created resources + ownership annotation
    let kube_server = tokio::spawn(async move {
        let mut created: i32 = 0;

        loop {
            mock_ingest_resources(&mut handle, remote_resources.clone(), &mut created).await;

            if created as usize == remote_resources.len() {
                break;
            }
        }

        // Ensure all resources created
        assert_eq!(created as usize, remote_resources.len());
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");
    let context = Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    );

    // The future loops endlessly, so the result here will show a lapsed interval
    let ingest_task = FakeResource::ingest_task(context);
    timeout(Duration::from_millis(50), ingest_task).await;

    kube_server.await;
    TEST_STORE.pin().clear();
}
