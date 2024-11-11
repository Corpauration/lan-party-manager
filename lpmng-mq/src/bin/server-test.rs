use std::sync::Arc;
use lpmng_mq::server::{AgentResponse, RouterRequest, Server};

#[tokio::main]
async fn main() {
    fn handler(_: RouterRequest, _: Arc<()>) -> AgentResponse {
        AgentResponse {
            success: true,
            body: "".into(),
        }
    }

    let _ = Server::new("[::1]:8080", handler, ()).serve().await;
}
