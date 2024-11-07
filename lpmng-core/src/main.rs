use biscuit_auth::KeyPair;
use tracing::{error, info};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use warp::Filter;

use api::{api_routes, public_route, ApiHandler};
use console::{console, ConsoleHandler, BANNER};
use lpmng_mq::client::Client;

mod api;
mod auth;
mod console;
mod db;
mod models;

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
        .with(if std::env::var("RUST_LOG").is_ok() { tracing_subscriber::EnvFilter::from_default_env() } else { tracing_subscriber::EnvFilter::new("info") })
        .init();

    let router_address = env_get("ROUTER_ADDRESS");
    let args: Vec<String> = std::env::args().collect();

    let console_mode: bool = if args.len() > 1 {
        args[1] == "console" || args[1] == "c"
    } else {
        false
    };

    if console_mode {
        println!("{}", BANNER);
        console(ConsoleHandler {
            db_handler: db::DbHandler::connect().await,
            router_address: router_address.clone(),
            router: Client::connect(&router_address).await,
        })
        .await;
    } else {
        let admin_key = env_get("ADMIN_KEY");
        let client_key = env_get("CLIENT_KEY");
        let port = match std::env::var("PORT") {
            Ok(p) => p.parse::<u16>().unwrap_or(3030),
            Err(_) => 3030,
        };
        println!("{}", BANNER);

        info!("api keys have been found");

        let db_handler = db::DbHandler::connect().await.unwrap();
        info!("database successfully connected");

        info!("http server starting...");
        warp::serve(
            public_route(env_get("PUBLIC_DIR")).or(api_routes(ApiHandler {
                db: db_handler,
                auth_key: KeyPair::new().private(),
                admin_key,
                client_key,
                router: Client::connect(&router_address)
                    .await
                    .expect("lpmng router has not been found"),
            })),
        )
        .run(([0, 0, 0, 0], port))
        .await;
    }
}
