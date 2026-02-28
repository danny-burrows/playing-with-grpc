mod hello_world {
    tonic::include_proto!("hello_world");
}

use hello_world::GreetUserRequest;
use hello_world::hello_world_client::HelloWorldClient;
use log::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let name = "Chuck Bradley".to_string();
    let remote_endpoint = "http://127.0.0.1:1234";

    let mut client = HelloWorldClient::connect(remote_endpoint).await?;
    debug!("created client: '{client:?}'");

    info!("sending grpc to '{remote_endpoint}' with name '{name}'");
    let request = tonic::Request::new(GreetUserRequest { name });
    debug!("request to send: '{request:?}'");

    let response = client.greet_user(request).await?;
    debug!("got response from server: '{response:?}'");

    let response_message = response.into_inner();
    debug!("response message: {response_message:?}");

    println!("{}", response_message.server_reply);

    Ok(())
}
