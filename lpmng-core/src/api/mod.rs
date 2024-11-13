use crate::auth::{check_admin, check_id, get_id};
use crate::db::DbHandler;
use crate::error::Error;
use crate::error::Error::{AuthorizationHeaderMalformed, RouterError};
use crate::error::Result;
use crate::mac::MacHandler;
use biscuit_auth::PrivateKey;
use lpmng_mq::client::agent::AgentResponse;
use std::convert::Infallible;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

mod devices;
mod login;
mod users;

pub struct ApiHandler {
    pub db: DbHandler,
    pub auth_key: PrivateKey,
    pub admin_key: String,
    pub router: Mutex<lpmng_mq::client::Client>,
    pub mac_handler: MacHandler,
}

fn is_admin(auth_token: String, private_key: &PrivateKey) -> Result<bool> {
    let mut split = auth_token.split(" ");

    if split.clone().count() != 2 {
        return Err(AuthorizationHeaderMalformed);
    }

    if split.clone().next().unwrap() != "Bearer" {
        return Err(AuthorizationHeaderMalformed);
    }

    check_admin(split.nth(1).unwrap().into(), private_key)
}

fn is_user(id: Uuid, auth_token: String, private_key: &PrivateKey) -> Result<bool> {
    let mut split = auth_token.split(" ");

    if split.clone().count() != 2 {
        return Err(AuthorizationHeaderMalformed);
    }

    if split.clone().next().unwrap() != "Bearer" {
        return Err(AuthorizationHeaderMalformed);
    }

    check_id(id, split.nth(1).unwrap().into(), private_key)
}

fn trace_router_response(res: AgentResponse) -> Result<()> {
    if res.success {
        debug!("router success");
        Ok(())
    } else {
        error!(error=%res.body, "router returned an error");
        Err(RouterError(res.body))
    }
}

fn with_handler(
    handler: Arc<ApiHandler>,
) -> impl Filter<Extract=(Arc<ApiHandler>,), Error=Infallible> + Clone {
    warp::any().map(move || handler.clone())
}

pub fn api_routes(
    handler: Arc<ApiHandler>,
) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    warp::path("api")
        .and(
            devices::routes(handler.clone())
                .or(users::routes(handler.clone()))
                .or(login::routes(handler.clone())),
        )
        .recover(Error::handle_warp_rejection)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["GET", "POST", "DELETE", "PATCH", "OPTIONS"])
                .allow_header("content-type")
                .allow_header("authorization")
        )
        .with(warp::trace(move |info| {
            let headers = info.request_headers();
            let ip = headers
                .get("X-Forwarded-For")
                .and_then(|e| e.to_str().ok())
                .and_then(|e| {
                    if e.contains(",") {
                        e.split(",").next()
                    } else {
                        Some(e)
                    }
                });
            let user_id = headers
                .get("Authorization")
                .and_then(|e| e.to_str().ok())
                .and_then(|e| e.strip_prefix("Bearer "))
                .and_then(|e| {
                    get_id(e.to_string(), &handler.auth_key)
                        .map_err(|error| error!(?error))
                        .ok()
                });
            let span = tracing::info_span!(
                "request",
                method = %info.method(),
                path = %info.path(),
                ip = tracing::field::Empty,
                user= tracing::field::Empty,
            );

            if let Some(ip) = ip {
                span.record("ip", ip);
            }
            if let Some(user_id) = user_id {
                span.record("user", &user_id);
            }

            span
        }))
}

pub fn public_route(
    public: String,
) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    if !Path::new(&public).exists() {
        error!(path = public, "unable to find the static html directory");
        panic!();
    }

    warp::get().and(warp::fs::dir(public))
}
