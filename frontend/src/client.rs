use blokhain_grpc::backend_client::BackendClient;
use grpc_web_client::Client;

pub mod blokhain_grpc {
    tonic::include_proto!("appserver");
}

async fn ping() {
    let client = Client::new("http://127.0.0.1:9991".to_string());
    let mut client = BackendClient::new(client);
    let response = client.ping(()).await;
    println!("RESPONSE={:?}", response);
}
