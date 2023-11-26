use hello::say_client::SayClient;
use hello::SayRequest;
use tonic::Request;
mod hello;

fn get_token() -> String {
   String::from("token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tls configuration
    let cert = include_str!("../certs/chat_client.pem"); // client pem key (.pem)
    let key = include_str!("../certs/chat_client.key");  // client key (.key)
    let id = tonic::transport::Identity::from_pem(cert.as_bytes(),key.as_bytes());
    let s = include_str!("../certs/my_chat_ca.pem");    // ca pem key (ca.pem)
    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls = tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .tls_config(tls) // use tls configuration on channel
        .connect()
        .await?;
    // get token for authentication of client
    let token = get_token(); // a method to get the token fn get_token(). This can be also a rpc call function
    // channel added with interceptor
    let mut client = SayClient::with_interceptor(channel, move |mut req:Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
            );
        Ok(req)
    });

    let request = tonic::Request::new(SayRequest {
            name:String::from("Rolly")
        },
    );
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
