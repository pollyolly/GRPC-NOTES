use futures::stream::iter;
use hello::say_client::SayClient;
use hello::SayRequest;
mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = SayClient::new(channel);
   // create a client
    let request = tonic::Request::new(iter(vec![
        SayRequest {
            name:String::from("Rolly")
        },
        SayRequest {
            name:String::from("Mitch")
        },
        SayRequest {
            name:String::from("Gloo")
        }
    ]));
    // calling rpc bidirectional function
    let mut response = client.bidirectional(request).await?.into_inner();
    // 
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    Ok(())
}
