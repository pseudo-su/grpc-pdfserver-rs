// use tonic::{transport::Server, Request, Response, Streaming, Status};
mod client;
mod pdfserverpb;

use tokio_stream::{Stream, StreamExt};

use pdfserverpb::greeter_client::{GreeterClient};
use pdfserverpb::files_client::{FilesClient};
use pdfserverpb::{HelloReply, HelloRequest, UploadFilesRequest, File, UploadFilesResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // say_hello().await?;
    upload_file().await?;

    Ok(())
}

async fn say_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut gclient = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = gclient.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn upload_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut fclient = FilesClient::connect("http://[::1]:50051").await?;

    let num = 1;
    let in_stream = client::fixtures::echo_requests_iter().take(num);

    let response = fclient.upload_files(in_stream).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
