use log::{debug, info};
use playing_with_gprc::hello_world::GreetUserRequest;
use playing_with_gprc::hello_world::hello_world_client::HelloWorldClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let name = "Chuck Bradley".to_string();
    let remote_endpoint = "http://127.0.0.1:1234";

    let mut client = HelloWorldClient::connect(remote_endpoint).await?;
    debug!("created client:\n{client:#?}");

    info!("sending grpc to '{remote_endpoint}' with name '{name}'");
    let request = tonic::Request::new(GreetUserRequest { name });
    debug!("request to send:\n{request:#?}");

    let response = client.greet_user(request).await?;
    debug!("got response from server:\n{response:#?}");

    let response_message = response.into_inner();
    debug!("response message:\n{response_message:#?}");

    info!("{}", response_message.server_reply);

    Ok(())
}
