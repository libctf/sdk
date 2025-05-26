use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate Challenges
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[doc(hidden)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Challenge",
    namespaced,
    doc = "Custom Kubernetes resource for 'Challenge' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate ChallengeRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[doc(hidden)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}
