use tonic::{transport::Server, Request, Response, Status};

use app_grpc::backend_server::{Backend, BackendServer};
use app_grpc::PingReply;

pub mod app_grpc {
    tonic::include_proto!("appserver");
}

use blokhain::blokhain::Blokhain;

#[derive(Debug, Default)]
pub struct MyBackend {}

#[tonic::async_trait]
impl Backend for MyBackend {
    async fn ping(
        &self,
        request: Request<()>,
    ) -> Result<Response<PingReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = app_grpc::PingReply {
            message: "Ok".to_string()
        };

        Ok(Response::new(reply))
    }

    async fn get_chain(
        &self,
        request: Request<()>,
    ) -> Result<Response<app_grpc::Chain>, Status> {
        println!("Got a request: {:?}", request);

        let reply = app_grpc::Chain {
            chain: [
                app_grpc::Block {
                    timestamp: "a".to_string(),
                    last_hash: "b".to_string(),
                    hash: "c".to_string(),
                    data: "d".to_string(),
                }
            ].to_vec()
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9991".parse()?;
    let backend = MyBackend::default();

    Server::builder()
        .add_service(BackendServer::new(backend))
        .serve(addr)
        .await?;

    Ok(())
}
