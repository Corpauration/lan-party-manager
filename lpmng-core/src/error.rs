use std::net::{AddrParseError, Ipv4Addr};
use tracing::error;

pub type Result<Ok> = core::result::Result<Ok, Error>;

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    DatabaseError(sqlx::Error),
    InvalidCredential,
    UserDoesNotExist,
    BiscuitError(biscuit_auth::error::Token),
    AuthorizationHeaderMalformed,
    Forbidden,
    BiscuitMalformed,
    NotRunningBehindAProxy,
    NotAnIp(AddrParseError),
    IoError(std::io::Error),
    RtnetlinkError(rtnetlink::Error),
    NoMacForThisIp(Ipv4Addr),
    FailedToExtractMac,
    RouterError(String),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::DatabaseError(value)
    }
}

impl From<biscuit_auth::error::Token> for Error {
    fn from(value: biscuit_auth::error::Token) -> Self {
        Error::BiscuitError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<rtnetlink::Error> for Error {
    fn from(value: rtnetlink::Error) -> Self {
        Error::RtnetlinkError(value)
    }
}

impl From<AddrParseError> for Error {
    fn from(value: AddrParseError) -> Self {
        Error::NotAnIp(value)
    }
}

impl warp::reject::Reject for Error {}

impl Error {
    pub async fn handle_warp_rejection(
        rejection: warp::Rejection,
    ) -> core::result::Result<impl warp::Reply, warp::Rejection> {
        if let Some(error) = rejection.find::<Error>() {
            error!(?error, "error while handling request");

            let res = match error {
                Error::DatabaseError(_)
                | Error::BiscuitError(_)
                | Error::BiscuitMalformed
                | Error::NotRunningBehindAProxy
                | Error::IoError(_)
                | Error::RtnetlinkError(_)
                | Error::NoMacForThisIp(_)
                | Error::FailedToExtractMac
                | Error::NotAnIp(_)
                | Error::RouterError(_) => warp::http::Response::builder()
                    .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body("")
                    .unwrap(),
                Error::InvalidCredential => warp::http::Response::builder()
                    .status(warp::http::StatusCode::BAD_REQUEST)
                    .body("Invalid username or password")
                    .unwrap(),
                Error::UserDoesNotExist => warp::http::Response::builder()
                    .status(warp::http::StatusCode::NOT_FOUND)
                    .body("User not found")
                    .unwrap(),
                Error::AuthorizationHeaderMalformed => warp::http::Response::builder()
                    .status(warp::http::StatusCode::BAD_REQUEST)
                    .body("Authorization header malformed")
                    .unwrap(),
                Error::Forbidden => warp::http::Response::builder()
                    .status(warp::http::StatusCode::FORBIDDEN)
                    .body("Forbidden")
                    .unwrap(),
            };

            Ok(res)
        } else {
            Err(rejection)
        }
    }
}
