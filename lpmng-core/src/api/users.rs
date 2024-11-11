use crate::api::{is_admin, is_user, trace_router_response, with_handler, ApiHandler};
use crate::auth::hash;
use crate::error::Error::{Forbidden, InvalidCredential, UserDoesNotExist};
use crate::model::device::Device;
use crate::model::user::{User, UserInput, UserPatch};
use chrono::Utc;
use futures::FutureExt;
use lpmng_mq::client::agent::RouterRequest;
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

async fn get_users(
    auth_token: String,
    handler: Arc<ApiHandler>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, &handler.auth_key)? {
        Err(Forbidden)?;
    }

    let res = handler.db.get_users().await?;

    Ok(warp::reply::json(
        &res.into_iter().map(User::into_view).collect::<Vec<_>>(),
    ))
}

async fn get_user(
    id: Uuid,
    auth_token: String,
    handler: Arc<ApiHandler>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let auth_key = &handler.auth_key;
    if !is_admin(auth_token.clone(), auth_key)? && !is_user(id, auth_token, auth_key)? {
        Err(Forbidden)?;
    }

    let res = handler
        .db
        .get_user(id)
        .await?
        .ok_or(UserDoesNotExist)?;
    Ok(warp::reply::json(&res.into_view()))
}

async fn create_user(
    user: UserInput,
    handler: Arc<ApiHandler>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if *user.username == "admin" {
        Err(InvalidCredential)?;
    }

    let mut user = user.into_unchecked();
    user.password = hash(user.password);
    handler.db.insert_user(user).await?;

    Ok(warp::reply())
}

async fn patch_user(
    user: UserPatch,
    auth_token: String,
    handler: Arc<ApiHandler>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, &handler.auth_key)? {
        Err(Forbidden)?;
    }

    let u = handler
        .db
        .get_user(user.id)
        .await?
        .ok_or(UserDoesNotExist)?;
    let new = User {
        id: user.id,
        username: user.username.map(|e| e.to_string()).unwrap_or(u.username),
        firstname: user.firstname.map(|e| e.to_string()).unwrap_or(u.firstname),
        lastname: user.lastname.map(|e| e.to_string()).unwrap_or(u.lastname),
        email: user.email.map(|e| e.to_string()).unwrap_or(u.email),
        password: u.password,
        phone: user.phone.map(|e| e.to_string()).unwrap_or(u.phone),
        role: user.role.map(|e| e.to_string()).unwrap_or(u.role),
        is_allowed: user.is_allowed.unwrap_or(u.is_allowed),
    };
    handler.db.update_user(new).await?;
    if user.is_allowed == Some(false) {
        let devices = handler.db.get_devices_by_user_id(user.id).await?;
        for device in devices {
            if device.internet {
                handler
                    .router
                    .lock()
                    .await
                    .send(RouterRequest {
                        action: "remove".to_string(),
                        body: device.mac.clone(),
                    })
                    .map(trace_router_response)
                    .await?;
                handler
                    .db
                    .update_device(Device {
                        id: device.id,
                        mac: device.mac,
                        user_id: device.user_id,
                        internet: false,
                        date_time: Utc::now().naive_utc(),
                    })
                    .await?;
            }
        }
    }
    if user.is_allowed == Some(true) {
        let devices = handler.db.get_devices_by_user_id(user.id).await?;
        for device in devices {
            if !device.internet {
                handler
                    .router
                    .lock()
                    .await
                    .send(RouterRequest {
                        action: "add".to_string(),
                        body: device.mac.clone(),
                    })
                    .map(trace_router_response)
                    .await?;
                handler
                    .db
                    .update_device(Device {
                        id: device.id,
                        mac: device.mac,
                        user_id: device.user_id,
                        internet: true,
                        date_time: Utc::now().naive_utc(),
                    })
                    .await?;
            }
        }
    }
    Ok(warp::reply())
}

pub async fn delete_user(
    user: UserPatch,
    auth_token: String,
    handler: Arc<ApiHandler>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_admin(auth_token, &handler.auth_key)? {
        Err(Forbidden)?;
    }

    let u = handler
        .db
        .get_user(user.id)
        .await?
        .ok_or(UserDoesNotExist)?;
    if u.is_allowed {
        let devices = handler.db.get_devices_by_user_id(user.id).await?;
        for device in devices {
            if device.internet {
                handler
                    .router
                    .lock()
                    .await
                    .send(RouterRequest {
                        action: "remove".to_string(),
                        body: device.mac.clone(),
                    })
                    .map(trace_router_response)
                    .await?;
            }
            handler.db.delete_device(device.id).await?;
        }
    }

    handler.db.delete_user(user.id).await?;
    Ok(warp::reply())
}

pub(super) fn routes(
    handler: Arc<ApiHandler>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let list = warp::get()
        .and(warp::path("users"))
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(get_users);

    let get = warp::get()
        .and(warp::path("users"))
        .and(warp::path::param())
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(get_user);

    let post = warp::post()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(with_handler(handler.clone()))
        .and_then(create_user);

    let patch = warp::patch()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(patch_user);

    let delete = warp::delete()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler))
        .and_then(delete_user);

    get.or(list).or(post).or(patch).or(delete)
}
