use k8s_openapi::{serde::{Deserialize, Serialize}};
use kube::{core::object::HasSpec, CustomResource};
use schemars::JsonSchema;

#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FakeAPIResource {
    pub id: i64,
    pub description: Option<String>,
}

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.test",
    version = "v1",
    kind = "FakeResource",
    derive = "Default",
    namespaced
)]
pub struct FakeResourceSpec {
    pub api_resource: FakeAPIResource,
}

/// CRD -> API
impl From<FakeResource> for FakeAPIResource {
    fn from(value: FakeResource) -> Self {
        value.spec().api_resource.clone()
    }
}

/// API -> CRD
impl From<FakeAPIResource> for FakeResource {
    fn from(api_resource: FakeAPIResource) -> Self {
        Self::new(&format!("fake-{}", api_resource.id), FakeResourceSpec { api_resource })
    }
}
