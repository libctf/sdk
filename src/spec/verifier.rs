use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate Verifiers
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[doc(hidden)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Verifier",
    namespaced,
    doc = "Custom Kubernetes resource for 'Verifier' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate VerifierRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[doc(hidden)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "VerifierRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'VerifierRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}
