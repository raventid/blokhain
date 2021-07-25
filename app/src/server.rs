use std::sync::{Arc, RwLock};
use std::cell::RefCell;

use tonic::{transport::Server, Request, Response, Status};

use app_grpc::backend_server::{Backend, BackendServer};
use app_grpc::PingReply;

pub mod app_grpc {
    tonic::include_proto!("appserver");
}

use blokhain::blokhain::Blokhain;

#[derive(Debug)]
pub struct MyBackend {
    bc: Arc<RwLock<Blokhain>>
}

impl MyBackend {
    pub fn new() -> Self {
        MyBackend {
            bc: Arc::new(RwLock::new(Blokhain::new(None)))
        }
    }
}

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

        println!("Rendering response with {:?}", self.bc.read().unwrap());

        let reply = app_grpc::Chain {
            chain: self.bc.read().expect("rwlock problem").chain.iter().map(|block| {
                app_grpc::Block {
                    timestamp: format!("{:?}", block.timestamp),
                    last_hash: format!("{:?}", block.last_hash),
                    hash: format!("{:?}", block.hash),
                    data: block.data.to_string(),
                }
            }).collect()
        };

        Ok(Response::new(reply))
    }

    async fn add_block(
        &self,
        request: Request<app_grpc::BlockData>,
    ) -> Result<Response<app_grpc::Confirmation>, Status> {
        println!("Got a request: {:?}", request);

        // I know it's weird I'm using first byte
        // I will change data type of block soon. Give
        // me just a little time
        let data = request.into_inner().payload.as_bytes()[0] - 48;
        self.bc.write().expect("rwlock problem").add_block(data);

        Ok(Response::new(app_grpc::Confirmation {
            status: "Block has been added".to_string()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9991".parse()?;
    let backend = MyBackend::new();

    Server::builder()
        .add_service(BackendServer::new(backend))
        .serve(addr)
        .await?;

    Ok(())
}
