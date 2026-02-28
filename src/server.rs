mod hello_world {
    tonic::include_proto!("hello_world");
}

use hello_world::hello_world_server::{HelloWorld, HelloWorldServer};
use hello_world::{GreetUserRequest, GreetUserResponse};
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
        println!("Got request: '{request:?}'");

        let message = request.into_inner();
        let name = message.name;

        let reply = GreetUserResponse {
            server_reply: format!("Hello {name}"),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1".parse()?;
    let hello_world_service = HelloWorldServer::new(HelloWorldService {});

    Server::builder()
        .add_service(hello_world_service)
        .serve(addr)
        .await?;

    Ok(())
}
