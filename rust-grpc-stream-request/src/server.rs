use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
// "hello" package name of chat.proto
use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
// include hello.rs
mod hello;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
// .. rest of rpcs
    // create new rpc to receive a stream
    // ReceieveStream function in chat.proto
     async fn receive_stream(&self, request:Request<tonic::Streaming<SayRequest>>)->Result<Response<SayResponse>,Status>{
        // converting request into stream
        let mut stream = request.into_inner();
        let mut message = String::from("");
        // listening on stream
        while let Some(req) = stream.message().await? {
            message.push_str(&format!("Hello {}\n", req.name))
        }
        // returning response
        Ok(Response::new(SayResponse {message}))
     }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // address
    let addr = "[::1]:50051".parse().unwrap();
    // create service
    let say = MySay::default();
    println!("Server listening on {}", addr);
    // adding our service to our server
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
