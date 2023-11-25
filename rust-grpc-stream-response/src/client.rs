use hello::say_client::SayClient;
use hello::SayRequest;
mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = SayClient::new(channel);
    let request = tonic::Request::new(SayRequest {
            name:String::from("John Mark")
        },
    );
    //the response is stream
    let mut response = client.send_stream(request).await?.into_inner();
    //listening to stream
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    Ok(())
}
