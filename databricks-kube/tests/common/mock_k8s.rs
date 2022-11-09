use crate::common::fake_resource::FakeResource;

use tower_test::mock::Handle;

use hyper::Body;
use k8s_openapi::http::{Request, Response};

use super::fake_resource::FakeAPIResource;

/**
 * Big thanks to this thread: https://github.com/kube-rs/kube/issues/429
 * @kazk's tower testing scaffold is extremely helpful!
 */

pub async fn mock_fake_resource_created(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    created_resource: FakeResource,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            let resource = match request.uri().query() {
                Some(q) if q.contains("watch") => {
                    serde_json::json!(
                        {
                            "type": "ADDED",
                            "object": created_resource,
                        }
                    )
                }
                _ => {
                    serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": ""
                            },
                            "items": []
                        }
                    )
                }
            };

            serde_json::to_vec(&resource).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            hyper::body::to_bytes(request.into_body())
                .await
                .unwrap()
                .into()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_fake_resource_updated_kube(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
    modified_resource: FakeResource,
    watch_update: Option<String>,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            let response = match request.uri().query() {
                Some(q) if q.contains("watch") => {
                    serde_json::json!(
                        {
                            "type": watch_update.unwrap_or("MODIFIED".to_string()),
                            "object": modified_resource
                        }
                    )
                }
                _ => {
                    serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": ""
                            },
                            "items": [
                                resource
                            ]
                        }
                    )
                }
            };

            serde_json::to_vec(&response).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            let parsed: FakeResource =
                serde_json::from_slice(&hyper::body::to_bytes(request.into_body()).await.unwrap())
                    .unwrap();

            assert_eq!(
                modified_resource.spec.api_resource,
                parsed.spec.api_resource,
            );

            serde_json::to_vec(&parsed).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_list_fake_resource(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
    assert_put: FakeAPIResource,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            match request.uri().query() {
                Some(q) if q.contains("watch") => vec![],
                _ => {
                    let list = serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": [
                                resource,
                            ]
                        }
                    );
                    serde_json::to_vec(&list).unwrap()
                }
            }
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/foo") => {
            let parsed: FakeResource =
                serde_json::from_slice(&hyper::body::to_bytes(request.into_body()).await.unwrap())
                    .unwrap();

            assert_eq!(assert_put, parsed.spec.api_resource);

            serde_json::to_vec(&parsed).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_fake_resource_deleted(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");
    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            match request.uri().query() {
                Some(q) if q.contains("watch") => {
                    let deleted = serde_json::json!(
                        {
                            "type": "DELETED",
                            "object": resource,
                        }
                    );
                    serde_json::to_vec(&deleted).unwrap()
                }
                _ => {
                    let list = serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": [resource]
                        }
                    );
                    serde_json::to_vec(&list).unwrap()
                }
            }
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}
