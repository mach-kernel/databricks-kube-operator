use crate::common::fake_resource::{FakeResource};

use flurry::HashMap;
use lazy_static::lazy_static;
use tower_test::mock::Handle;

use hyper::body::HttpBody;
use hyper::Body;
use k8s_openapi::http::{Request, Response};
use tower_test::mock;

use super::fake_resource::FakeAPIResource;

/**
 * Big thanks to this thread: https://github.com/kube-rs/kube/issues/429
 * @kazk's tower testing scaffold is extremely helpful!
 */

pub async fn mock_fake_resource_created(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    created_resource: FakeResource
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
                            "object": {
                                "apiVersion": "com.dstancu.test/v1",
                                "kind": "FakeResource",
                                "metadata": {
                                    "name": "test",
                                    "resourceVersion": "2",
                                },
                                "spec": {
                                    "api_resource": {
                                        "id": 1
                                    }
                                }
                            }
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

pub async fn mock_fake_resource_updated(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
    modified_resource: FakeResource,
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
                            "type": "MODIFIED",
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

            serde_json::to_vec(&resource).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            let parsed: FakeResource = serde_json::from_slice(
                &hyper::body::to_bytes(request.into_body()).await.unwrap()
            ).unwrap();

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

pub async fn serve_fake_resource(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
    assert_resource: FakeAPIResource,
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
                    let resource = serde_json::json!(
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
                    serde_json::to_vec(&resource).unwrap()
                }
            }
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/foo") => {
            let parsed: FakeResource = serde_json::from_slice(
                &hyper::body::to_bytes(request.into_body()).await.unwrap()
            ).unwrap();

            assert_eq!(
                assert_resource,
                parsed.spec.api_resource,
            );

            serde_json::to_vec(&parsed).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}