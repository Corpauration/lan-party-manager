pub use agent::{
    router_server::{Router, RouterServer},
    AgentResponse, PingRequest, RouterRequest,
};
use std::sync::Arc;
use tonic::{transport, Request, Response, Status};

pub mod agent {
    tonic::include_proto!("agent");
}

impl AgentResponse {
    pub fn success() -> Self {
        AgentResponse {
            success: true,
            body: "".into(),
        }
    }

    pub fn fail(str: &str) -> Self {
        AgentResponse {
            success: false,
            body: str.into(),
        }
    }
}

#[derive(Debug)]
pub struct RouterService<Context: Sync + Send> {
    pub handler: fn(RouterRequest, Arc<Context>) -> AgentResponse,
    pub ctx: Arc<Context>,
}

impl Default for RouterService<()> {
    fn default() -> Self {
        fn handler(_: RouterRequest, _: Arc<()>) -> AgentResponse {
            AgentResponse {
                success: false,
                body: "Unimplemented Server Handler".into(),
            }
        }

        RouterService {
            handler,
            ctx: Arc::new(()),
        }
    }
}

#[tonic::async_trait]
impl<Context: Sync + Send + 'static> Router for RouterService<Context> {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<AgentResponse>, Status> {
        if request.into_inner().body == "ping" {
            return Ok(Response::new(AgentResponse {
                success: true,
                body: "PONG !".into(),
            }));
        }

        Ok(Response::new(AgentResponse {
            success: false,
            body: "PONG ?".into(),
        }))
    }

    async fn send(
        &self,
        request: Request<RouterRequest>,
    ) -> Result<Response<AgentResponse>, Status> {
        Ok(Response::new((self.handler)(
            request.into_inner(),
            self.ctx.clone(),
        )))
    }
}

pub struct Server<Context: Sync + Send> {
    address: String,
    handler: fn(RouterRequest, Arc<Context>) -> AgentResponse,
    ctx: Arc<Context>,
}

impl<Context: Sync + Send + 'static> Server<Context> {
    pub fn new(
        address: &str,
        handler: fn(RouterRequest, Arc<Context>) -> AgentResponse,
        ctx: Context,
    ) -> Self {
        Server {
            address: address.into(),
            handler,
            ctx: Arc::new(ctx),
        }
    }

    async fn _serve(
        &self,
        service: RouterService<Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        transport::Server::builder()
            .add_service(RouterServer::new(service))
            .serve(self.address.parse().unwrap())
            .await?;
        Ok(())
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._serve(RouterService::<Context> {
            handler: self.handler,
            ctx: self.ctx.clone(),
        })
        .await
    }
}
