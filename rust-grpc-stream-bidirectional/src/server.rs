use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
mod hello;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
     // defining return stream
    type BidirectionalStream = mpsc::Receiver<Result<SayResponse,Status>>;

    //Send function in .proto
     async fn bidirectional(&self, request:Request<tonic::Streaming<SayRequest>>
                          )->Result<Response<Self::BidirectionalStream>,Status>{
         // convert request into stream
        let mut streamer = request.into_inner();
        // creating queue, four channel
        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            // listening on request stream
            // for _ in 0..4 {
            while let Some(req) = streamer.message().await.unwrap(){
                tx.send(Ok(SayResponse {
                   message: format!("Hello {}", req.name), 
                }))
                .await;
            }
            // }
        });
        // returning stream as receiver
        Ok(Response::new(rx))
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
