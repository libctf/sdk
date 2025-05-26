use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate Flags
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Flag",
    namespaced,
    doc = "Custom Kubernetes resource for 'Flag' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate FlagRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "FlagRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'FlagRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}
