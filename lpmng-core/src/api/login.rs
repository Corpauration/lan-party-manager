use crate::api::{with_handler, ApiHandler};
use crate::auth::build_token;
use crate::model::login::{Credentials, Login};
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

async fn login(login: Login, handler: Arc<ApiHandler>) -> Result<impl Reply, Rejection> {
    if *login.username == "admin" && *login.password == handler.admin_key {
        let t = build_token("admin".into(), Uuid::nil(), &handler.auth_key)?;
        return Ok(warp::reply::json(&Credentials {
            biscuit: t,
            role: "admin".into(),
            user_id: Uuid::nil(),
        }));
    }

    let auth = handler
        .db
        .check_password(login.username.to_string(), login.password.to_string())
        .await?;

    let (role, id) = auth;

    let t = build_token(role.to_owned(), id, &handler.auth_key)?;

    Ok(warp::reply::json(&Credentials {
        biscuit: t,
        role,
        user_id: id,
    }))
}

pub fn routes(
    handler: Arc<ApiHandler>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_handler(handler))
        .and_then(login)
}
