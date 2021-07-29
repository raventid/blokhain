use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::{mpsc, RwLock};
use std::pin::Pin;

use std::cell::RefCell;

use tonic::{transport::Server, Request, Response, Status};
use futures::{Stream, StreamExt};

use app_grpc::backend_server::{Backend, BackendServer};
use app_grpc::PingReply;

pub mod app_grpc {
    tonic::include_proto!("appserver");
}

use blokhain::blokhain::Blokhain;

#[derive(Debug)]
pub struct MyBackend {
    bc: Arc<std::sync::RwLock<Blokhain>>,
    subscriptions: Arc<RwLock<Shared>>
}

impl MyBackend {
    pub fn new() -> Self {
        MyBackend {
            bc: Arc::new(std::sync::RwLock::new(Blokhain::new(None))),
            subscriptions: Arc::new(RwLock::new(Shared::new()))
        }
    }
}

// When a new user connects, we will create a pair of mpsc channel.
// Add the users and its related senders will be saved in below shared struct
#[derive(Debug)]
struct Shared {
    senders: HashMap<String, mpsc::Sender<app_grpc::Message>>,
}
impl Shared {
    fn new() -> Self {
        Shared {
            senders: HashMap::new(),
        }
    }

    async fn broadcast(&self, msg: app_grpc::Message) {
        // To make our logic simple and consistency, we will broadcast to all
        // users which include msg sender.
        // On frontend, sender will send msg and receive its broadcasted msg
        // and then show his msg on frontend page.
        for (name, tx) in &self.senders {
            match tx.send(msg.clone()).await {
                Ok(_) => {}
                Err(_) => {
                    println!("[Broadcast] SendError: to {}, {:?}", name, msg)
                }
            }
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

    type connectServerStream =
        Pin<Box<dyn Stream<Item = Result<app_grpc::Message, Status>> + Send + Sync + 'static>>;

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

    async fn connect_server(
        &self,
        request: Request<app_grpc::Registration>,
    ) -> Result<Response<Self::connectServerStream>, Status> {
        let name = request.into_inner().user_name;
        let (stream_tx, stream_rx) = mpsc::channel(1); // Fn usage

        // When connecting, create related sender and reciever
        let (tx, mut rx) = mpsc::channel(1);
        {
            self.subscriptions.write().await.senders.insert(name.clone(), tx);
        }

        let subscriptions = self.subscriptions.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        // If sending failed, then remove the user from shared data
                        println!(
                            "[Remote] stream tx sending error. Remote {}",
                            &name
                        );
                        subscriptions.write().await.senders.remove(&name);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(stream_rx)) as Self::connectServerStream))
    }

    async fn exchange(
        &self,
        request: Request<app_grpc::Message>,
    ) -> Result<Response<()>, Status> {
        println!("Stream path has been hitten");

        let message = app_grpc::Message { msg: request.into_inner().msg };

        dbg!(self.subscriptions.read().await);

        self.subscriptions.read().await.broadcast(message).await;

        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9991".parse()?;
    let backend = BackendServer::new(MyBackend::new());

    // Server::builder()
    //     .add_service(BackendServer::new(backend))
    //     .serve(addr)
    //     .await?;

    Server::builder()
       .accept_http1(true)
       .add_service(tonic_web::enable(backend))
       .serve(addr)
       .await?;

    Ok(())
}
