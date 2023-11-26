use tonic::{transport::Server, Request, Response, Status};
// proto Say {} class
use hello::say_server::{Say, SayServer};
// .proto messages
use hello::{SayRequest, SayResponse};
// hello.rs
mod hello;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    //Send function in .proto
     async fn send(&self, request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{
          Ok(Response::new(SayResponse{
               message:format!("hello {}", request.get_ref().name),
          }))
     }
}
// Authenticator function
fn interceptor(req: Request<()>)->Result<Request<()>,Status>{
   let token = match req.metadata().get("authorization"){
      Some(token)=>token.to_str(),
      None=>return Err(Status::unauthenticated("Token not found"))
   };
   Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tls configuration
    let cert = include_str!("../certs/chat_server.pem"); // client pem key (.pem)
    let key = include_str!("../certs/chat_server.key");  // client key (.key)
    let id = tonic::transport::Identity::from_pem(cert.as_bytes(),key.as_bytes());
    let s = include_str!("../certs/my_chat_ca.pem");    // ca pem key (ca.pem)
    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls = tonic::transport::ServerTlsConfig::new().identity(id).client_ca_root(ca);

    //address
    let addr = "[::1]:50051".parse().unwrap();
    //create service
    let say = MySay::default();
    // call authenticator function
    // intercepter function  
    let ser = SayServer::with_interceptor(say, interceptor);
    println!("Server listening on {}", addr);
    //adding our service to our server
    Server::builder()
        .tls_config(tls)
        .add_service(ser)  // ser, variable added with interceptor
        .serve(addr)
        .await?;
    Ok(())
}

