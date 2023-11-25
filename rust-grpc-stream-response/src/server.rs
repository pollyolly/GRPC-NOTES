use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
mod hello;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {

    type SendStreamStream = mpsc::Receiver<Result<SayResponse,Status>>;

    //Send function in .proto
     async fn send_stream(&self, request:Request<SayRequest>)->Result<Response<Self::SendStreamStream>,Status>{
        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for _ in 0..4 {
                tx.send(Ok(SayResponse {
                   message: format!("hello"), 
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
     }
     async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse {
            message: format!("hello {}", request.get_ref().name),
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
