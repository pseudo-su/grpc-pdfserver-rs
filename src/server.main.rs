mod client;
mod pdfserverpb;

use tokio_stream::{Stream, StreamExt};
use tonic::{IntoRequest, IntoStreamingRequest};
use tonic::{transport::Server, Request, Response, Streaming, Status};

use pdfserverpb::greeter_server::{Greeter, GreeterServer};
use pdfserverpb::files_server::{Files, FilesServer};
use pdfserverpb::{HelloReply, HelloRequest, UploadFilesRequest, UploadFilesResponse};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = pdfserverpb::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct MyFiles {}

#[tonic::async_trait]
impl Files for MyFiles {
    async fn upload_files(
        &self,
        request: Request<Streaming<UploadFilesRequest>>,
    ) -> Result<Response<UploadFilesResponse>, Status> {
        println!("request: {:?}", request);

        let mut in_stream = request.into_inner();

        println!("size {:?}", in_stream.size_hint());

        while let Some(result) = in_stream.next().await {
            println!("chunk {:?}", result.unwrap());
        }

        let reply = pdfserverpb::UploadFilesResponse {};
        println!("reply: {:?}", reply);

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let my_greeter_impl = MyGreeter::default();
    let my_files_impl = MyFiles::default();

    Server::builder()
        .add_service(FilesServer::new(my_files_impl))
        .add_service(GreeterServer::new(my_greeter_impl))
        .serve(addr)
        .await?;

    Ok(())
}
