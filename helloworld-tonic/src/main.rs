use hello_world::greeter_server::Greeter;
use hello_world::greeter_server::GreeterServer;
use hello_world::HelloReply;
use hello_world::HelloRequest;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request; {:?}", request);
        let reply = hello_world::HelloReply {
            message: format!("Hello {}", request.into_inner().name).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
