use crate::api::{is_admin, is_user, trace_router_response, with_handler, ApiHandler};
use crate::error::Error;
use crate::error::Error::{Forbidden, NotRunningBehindAProxy, UserDoesNotExist};
use crate::model::device::{Device, DeviceInput, NewDevice};
use chrono::Utc;
use futures::FutureExt;
use lpmng_mq::client::agent::RouterRequest;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

async fn get_devices(
    auth_token: String,
    handler: Arc<ApiHandler>,
) -> Result<impl Reply, Rejection> {
    if !is_admin(auth_token, &handler.auth_key)? {
        Err(Forbidden)?;
    }

    let res = handler.db.get_devices().await?;

    Ok(warp::reply::json(&res))
}

async fn get_device_by_user(
    id: Uuid,
    auth_token: String,
    handler: Arc<ApiHandler>,
) -> Result<impl Reply, Rejection> {
    let auth_key = &handler.auth_key;
    if !is_admin(auth_token.clone(), auth_key)? && !is_user(id, auth_token, auth_key)? {
        Err(Forbidden)?;
    }

    Ok(warp::reply::json(
        &handler.db.get_devices_by_user_id(id).await?,
    ))
}

pub async fn add_device(
    device: DeviceInput,
    auth_token: String,
    ip: String,
    handler: Arc<ApiHandler>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_user(device.user_id, auth_token, &handler.auth_key)? {
        Err(Forbidden)?;
    }

    let ip = if ip.contains(",") {
        ip.split(",").next().unwrap().to_string()
    } else {
        ip
    };
    if ip.is_empty() {
        Err(NotRunningBehindAProxy)?;
    }

    let mac = handler
        .mac_handler
        .get_mac_from_ip(Ipv4Addr::from_str(&ip).map_err(Into::<Error>::into)?)
        .await?;

    let device = NewDevice {
        mac: mac.clone(),
        user_id: device.user_id,
        internet: false,
        date_time: Utc::now().naive_utc(),
    };

    let old_device = handler.db.get_device_by_mac(device.mac.clone()).await?;

    if let Some(old_device) = &old_device {
        if old_device.user_id != device.user_id {
            error!(
                current_user = device.user_id.as_hyphenated().to_string(),
                existing_device_user = old_device.user_id.as_hyphenated().to_string(),
                device_mac = device.mac,
                "dubious: device does not belong to the current user"
            );
            Err(Forbidden)?;
        }
    }

    let authorized = handler
        .db
        .get_user(device.user_id)
        .await?
        .ok_or(UserDoesNotExist)?
        .is_allowed;

    match old_device {
        None => {
            handler
                .router
                .lock()
                .await
                .send(RouterRequest {
                    action: "add".to_string(),
                    body: mac.clone(),
                })
                .map(trace_router_response)
                .await?;
            handler
                .db
                .insert_device(NewDevice {
                    mac: mac.clone(),
                    user_id: device.user_id,
                    internet: authorized,
                    date_time: Utc::now().naive_utc(),
                })
                .await?;
        }
        Some(old_device) => {
            match (authorized, old_device.internet) {
                (false, true) => {
                    handler
                        .router
                        .lock()
                        .await
                        .send(RouterRequest {
                            action: "remove".to_string(),
                            body: mac.clone(),
                        })
                        .map(trace_router_response)
                        .await?;
                }
                (true, false) => {
                    handler
                        .router
                        .lock()
                        .await
                        .send(RouterRequest {
                            action: "add".to_string(),
                            body: mac.clone(),
                        })
                        .map(trace_router_response)
                        .await?;
                }
                _ => {}
            }

            if authorized != old_device.internet {
                handler
                    .db
                    .update_device(Device {
                        id: old_device.id,
                        mac: mac.clone(),
                        user_id: old_device.user_id,
                        internet: authorized,
                        date_time: Utc::now().naive_utc(),
                    })
                    .await?;
            }
        }
    }

    Ok(warp::reply())
}

pub(super) fn routes(
    handler: Arc<ApiHandler>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let list = warp::get()
        .and(warp::path("devices"))
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(get_devices);

    let get = warp::get()
        .and(warp::path("devices"))
        .and(warp::path::param())
        .and(warp::header::<String>("Authorization"))
        .and(with_handler(handler.clone()))
        .and_then(get_device_by_user);

    let post = warp::post()
        .and(warp::path("devices"))
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(warp::header::<String>("X-Forwarded-For"))
        .and(with_handler(handler))
        .and_then(add_device);

    get.or(list).or(post)
}
