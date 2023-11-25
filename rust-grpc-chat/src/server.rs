use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
mod hello;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    //Send function in chat.proto
     async fn send(&self, request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{
          Ok(Response::new(SayResponse{
               message:format!("hello {}", request.get_ref().name),
          }))
     }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //address
    let addr = "[::1]:50051".parse().unwrap();
    //create service
    let say = MySay::default();
    println!("Server listening on {}", addr);
    //adding our service to our server
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
