mod error;
mod nfables;

use crate::nfables::Nftables;
use lpmng_mq::server::{AgentResponse, RouterRequest, Server};
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const HEX: [char; 22] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F',
];

fn is_mac_valid(str: &str) -> bool {
    let parts = str.split(":").collect::<Vec<_>>();
    if parts.len() != 6 {
        return false;
    }
    !parts
        .into_iter()
        .any(|part| {
            if part.len() != 2 {
                return true;
            }
            for e in part.chars() {
                if !HEX.contains(&e) {
                    return true;
                }
            }

            false
        })
}

fn server_handler(req: RouterRequest, nftables: Arc<Nftables>) -> AgentResponse {
    match req.action.as_str() {
        "add" => {
            let mac = req.body;

            if !is_mac_valid(&mac) {
                error!(%mac, "invalid mac address");
                return AgentResponse::fail("unable to parse mac address");
            }

            info!(%mac, "adding mac address");

            match nftables.add_items_in_set(vec![mac.clone()]) {
                Ok(_) => AgentResponse::success(),
                Err(error) => {
                    error!(?error, %mac, "failed to add mac address");
                    AgentResponse::fail(&format!("{error:?}"))
                }
            }
        }
        "remove" => {
            let mac = req.body;

            if !is_mac_valid(&mac) {
                error!(%mac, "invalid mac address");
                return AgentResponse::fail("unable to parse mac address");
            }

            info!(%mac, "removing mac address");

            match nftables.delete_items_in_set(vec![mac.clone()]) {
                Ok(_) => AgentResponse::success(),
                Err(error) => {
                    error!(?error, %mac, "failed to remove mac address");
                    AgentResponse::fail(&format!("{error:?}"))
                }
            }
        }
        "get" => {
            info!("getting mac addresses");

            match nftables.get_items_in_set() {
                Ok(body) => AgentResponse {
                    success: true,
                    body: body.join("\n"),
                },
                Err(error) => {
                    error!(?error, "failed to get mac addresses");
                    AgentResponse::fail(&format!("{error:?}"))
                }
            }
        }
        "clear" => {
            info!("clearing mac addresses");

            match nftables.flush_set() {
                Ok(_) => AgentResponse::success(),
                Err(error) => {
                    error!(?error, "failed to clear mac addresses");
                    AgentResponse::fail(&format!("{error:?}"))
                }
            }
        }
        _ => AgentResponse {
            success: false,
            body: "unknown method".into(),
        },
    }
}

fn server_handler_test(req: RouterRequest, _: Arc<()>) -> AgentResponse {
    match req.action.as_str() {
        "add" => {
            let mac = req.body;

            if !is_mac_valid(&mac) {
                error!(%mac, "invalid mac address");
                return AgentResponse::fail("unable to parse mac address");
            }

            info!(%mac, "adding mac address");
            AgentResponse::success()
        }
        "remove" => {
            let mac = req.body;

            if !is_mac_valid(&mac) {
                error!(%mac, "invalid mac address");
                return AgentResponse::fail("unable to parse mac address");
            }

            info!(%mac, "removing mac address");
            AgentResponse::success()
        }
        "get" => {
            info!("getting mac addresses");
            AgentResponse::success()
        }
        "clear" => {
            info!("clearing mac addresses");
            AgentResponse::success()
        }
        _ => AgentResponse {
            success: false,
            body: "unknown method".into(),
        },
    }
}

fn env_abort(env: &'static str) -> impl Fn(std::env::VarError) -> String {
    move |e| {
        error!(error=?e, "{env} is not set");
        std::process::exit(1);
    }
}

fn env_get(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(env_abort(env))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::EnvFilter::from_default_env()
        } else {
            tracing_subscriber::EnvFilter::new("info")
        })
        .init();

    let router_address = env_get("ROUTER_ADDRESS");

    if std::env::var("TEST").is_ok() {
        info!("starting server in test mode");
        let server = Server::new(&router_address, server_handler_test, ());

        info!("server has started");
        let _ = server.serve().await;
    } else {
        info!("starting server in production mode");

        let nf_table = env_get("NF_TABLE");
        let nf_set = env_get("NF_SET");
        let nftables = Nftables::new(nf_table, nf_set);

        let server = Server::new(&router_address, server_handler, nftables);

        info!("server has started");
        if let Err(error) = server.serve().await {
            error!(?error, "Server crashed!");
        }
    }
}
