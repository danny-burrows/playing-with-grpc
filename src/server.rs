mod hello_world {
    tonic::include_proto!("hello_world");
}

use std::net::{Ipv4Addr, SocketAddrV4};

use hello_world::hello_world_server::{HelloWorld, HelloWorldServer};
use hello_world::{GreetUserRequest, GreetUserResponse};
use log::{debug, info};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Clone)]
struct HelloWorldService {}

#[tonic::async_trait]
impl HelloWorld for HelloWorldService {
    async fn greet_user(
        &self,
        request: Request<GreetUserRequest>,
    ) -> Result<Response<GreetUserResponse>, Status> {
        debug!("got request: '{request:?}'");

        let message = request.into_inner();
        debug!("message in request: '{message:?}'");
        let name = message.name;
        info!("client name is '{name}'");

        println!("Oh, '{name}' ey?");

        let server_reply = format!("Hello {name}");
        info!("will reply with '{server_reply}'");

        let reply = GreetUserResponse { server_reply };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let hello_world_service = HelloWorldServer::new(HelloWorldService {});

    let addr = SocketAddrV4::new(Ipv4Addr::from_octets([127, 0, 0, 1]), 1234);
    info!("serving grpc on '{addr}'");
    Server::builder()
        .add_service(hello_world_service)
        .serve(addr.into())
        .await?;

    Ok(())
}
