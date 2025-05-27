use crate::spec::challenge::{Challenge, ChallengeSpec};
// use crate::spec::util::{ResourceRequestSpec, ResourceSpec};
use context::Context;
use hyper_util::rt::TokioExecutor;
use kube::Error;
use kube::api::{DeleteParams, GetParams, Patch, PatchParams, PostParams};
use kube::{Client, Config, client::ConfigExt};
use serde::Serialize;
use std::future::Future;
use tower::{BoxError, ServiceBuilder};
use tracing::{Level, span};

pub const MAX_RETRIES: u32 = 3;

/// Defines an interface for creating, reading, updating, and deleting (CRUD) resources.
pub trait Crud {
    // Challenge, Plugin, Flag, Verifier
    /// Expects same output as `kube::api::Request::create()`
    fn create(
        ctx: &Context,
        client: Client,
        pp: &PostParams,
        data: ChallengeSpec,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send;
    /// Expects same output as `kube::api::Request::get()`
    fn read(
        ctx: &Context,
        client: Client,
        name: String,
        gp: &GetParams,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send;
    /// Expects same output as `kube::api::Request::patch()`
    fn update<P: Serialize + std::fmt::Debug + Send + Sync + Clone + 'static>(
        ctx: &Context,
        client: Client,
        name: String,
        pp: &PatchParams,
        patch: &Patch<P>,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send;
    /// Expects same output as `kube::api::Request::delete()`
    fn delete(
        ctx: &Context,
        client: Client,
        name: String,
        dp: &DeleteParams,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send;
}

/// Wrapper around `kube::api::Request`, creates a 'client' for working on Kubernetes resources.
pub async fn client() -> Result<Client, BoxError> {
    let span = span!(Level::TRACE, "create_client");
    let _enter = span.enter();
    let config = Config::infer().await?;
    let service = ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .map_err(BoxError::from)
        .service(hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build_http());
    Ok(Client::new(service, config.default_namespace))
}
