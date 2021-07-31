use blokhain_grpc::backend_client::BackendClient;
use grpc_web_client::Client;
use tonic::Streaming;

pub mod blokhain_grpc {
    tonic::include_proto!("appserver");
}

pub async fn ping() -> String {
    let client = Client::new("http://127.0.0.1:9991".to_string());
    let mut client = BackendClient::new(client);
    let response = client.ping(()).await;
    format!("RESPONSE={:?}", response)
}

pub async fn connect_server() -> Streaming<blokhain_grpc::Block> {
    let client = Client::new("http://127.0.0.1:9991".to_string());
    let mut client = BackendClient::new(client);
    let registration_message = blokhain_grpc::Registration {
        user_name: "browser".to_string()
    };
    let stream = client.connect_server(registration_message).await.unwrap().into_inner();
    stream
}
