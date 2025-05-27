use crate::api::utils::{Crud, MAX_RETRIES};
use crate::spec::challenge::{Challenge, ChallengeSpec};
use context::Context;
use either::Either;
use futures::future::Future;
use kube::Api;
use kube::Error;
use kube::api::{DeleteParams, GetParams, Patch, PatchParams, PostParams, WatchParams};
use log;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{Level, event, span};

pub struct Resource {
    spec: Challenge,
}

impl Resource {
    pub fn new(name: &String) -> Self {
        Resource {
            spec: Challenge::new(
                name,
                ChallengeSpec {
                    name: name.into(),
                    info: "foobar".into(),
                    replicas: 1,
                },
            ),
        }
    }
}

impl Crud for Resource {
    /// Create a challenge resource.
    ///
    /// Simply a wrapper around the `kube::Api::create()` method.
    fn create(
        ctx: &Context,
        client: kube::Client,
        pp: &PostParams,
        data: ChallengeSpec,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send {
        let span = span!(Level::TRACE, "create_challenge");
        let _enter = span.enter();
        let pp = pp.clone();
        async move {
            let challenge: Api<Challenge> = Api::namespaced(client, "default");
            let c = Challenge::new("example", data);
            challenge.create(&pp, &c).await
        }
    }

    /// Read a challenge resource.
    ///
    /// Simply a wrapper around the `kube::Api::get()` method.
    fn read(
        ctx: &Context,
        client: kube::Client,
        name: String,
        gp: &GetParams,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send {
        let span = span!(Level::TRACE, "get_challenge");
        let _enter = span.enter();
        let gp = gp.clone();
        async move {
            let challenge: Api<Challenge> = Api::namespaced(client, "default");
            challenge.get_opt(&name).await?.ok_or_else(|| {
                kube::Error::Api(kube::error::ErrorResponse {
                    status: "NotFound".to_string(),
                    message: format!("Challenge '{}' not found", name),
                    reason: "NotFound".to_string(),
                    code: 404,
                })
            })
        }
    }

    /// Update a challenge resource.
    ///
    /// Simply a wrapper around the `kube::Api::patch()` method.
    fn update<P: Serialize + std::fmt::Debug>(
        ctx: &Context,
        client: kube::Client,
        name: String,
        params: &PatchParams,
        patch: &Patch<P>,
    ) -> impl Future<Output = Result<Challenge, kube::Error>> + Send
    where
        P: Send + Sync + Clone + 'static,
    {
        let span = span!(Level::TRACE, "update_challenge");
        let _enter = span.enter();
        let params = params.clone();
        let patch = patch.clone();
        async move {
            let challenge: Api<Challenge> = Api::namespaced(client, "default");
            challenge.patch(&name, &params, &patch).await
        }
    }

    /// Delete a challenge resource.
    ///
    /// A looped wrapper around the `kube::Api::delete()` method.
    /// Loops until the resource is deleted or an error occurs.
    fn delete(
        ctx: &Context,
        client: kube::Client,
        name: String,
        dp: &DeleteParams,
    ) -> impl Future<Output = Result<Challenge, Error>> + Send {
        let span = span!(Level::TRACE, "delete_challenge");
        let _enter = span.enter();
        let dp = dp.clone(); // Clone DeleteParams since we need to move it

        async move {
            let challenge: Api<Challenge> = Api::namespaced(client, "default");
            let mut retries = 0;

            loop {
                match challenge.delete(&name, &dp).await? {
                    Either::Left(challenge_obj) => {
                        return Ok(challenge_obj);
                    }
                    Either::Right(status) => {
                        event!(Level::INFO, "Deleted CRD status");
                        log::info!(
                            "Deleted CRD status: {:?}, retry {}/{}...",
                            status,
                            retries + 1,
                            MAX_RETRIES
                        );

                        retries += 1;
                        if retries >= MAX_RETRIES {
                            event!(Level::ERROR, "Error 429: TooManyRetries");
                            return Err(kube::Error::Api(kube::error::ErrorResponse {
                                status: "TooManyRetries".to_string(),
                                message: format!("Max retries ({}) exceeded", MAX_RETRIES),
                                reason: "TooManyRetries".to_string(),
                                code: 429,
                            }));
                        }

                        sleep(Duration::from_millis(100)).await; // Use tokio::time::sleep, not thread::sleep
                        continue;
                    }
                }
            }
        }
    }
}

// impl Challenge {
//     pub fn new(
//         id: String,
//         name: String,
//         description: String,
//         points: u32,
//         category: String,
//         flag: String,
//     ) -> Self {
//         Challenge {
//             id,
//             name,
//             description,
//             points,
//             category,
//             flag,
//             client: client().unwrap(),
//         }
//     }
//     pub fn watch(&self, wp: &WatchParams, ver: &str) -> Result<Request<Vec<u8>>, Error> {
//         self.client.watch(wp, ver)
//     }
//     pub fn replace(
//         &self,
//         name: String,
//         pp: &PostParams,
//         data: Vec<u8>,
//     ) -> Result<Request<Vec<u8>>, Error> {
//         self.client.replace(&name, pp, data)
//     }
// }
