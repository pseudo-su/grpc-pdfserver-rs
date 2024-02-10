use tokio_stream::{Stream, StreamExt};

use crate::pdfserverpb::{HelloReply, HelloRequest, UploadFilesRequest, File, UploadFilesResponse};

// use bytes::BytesMut;
// use futures::{Future, Stream};
// use tokio_codec::{BytesCodec, FramedRead};
use tokio::fs;
use tokio_util::codec::{BytesCodec, FramedRead};
// use tokio::runtime::Runtime;
// futures::stream::
// fn main() {
//     let mut rt = Runtime::new().unwrap();
//     let file = fs::File::open("Cargo.toml");
//     let task = file.and_then(|file| {
//         FramedRead::new(file, BytesCodec::new())
//             .map(BytesMut::freeze) // Map stream of `BytesMut` to stream of `Bytes`
//             .concat2()
//             .and_then(|bytes| {
//                 assert_eq!(bytes, &include_bytes!("../../Cargo.toml")[..]);
//                 Ok(())
//             })
//     });

//     if let Err(e) = rt.block_on(task) {
//         dbg!(e);
//     }
// }

// pub fn read_file() -> impl Stream<Item = UploadFilesRequest> {
//     panic!()
//     // return tokio_stream::new();
// }

// pub async fn read_file(filename: &str) -> Result<impl Stream<Item = UploadFilesRequest>, Box<dyn std::error::Error>> {
//     // Serve a file by asynchronously reading it by chunks using tokio-util crate.

//     if let Ok(file) = fs::File::open(filename).await {
//         let stream = FramedRead::new(file, BytesCodec::new());
//         return Ok(stream);
//     }
//     std::fmt::Error{}
// }

pub fn echo_requests_iter() -> impl Stream<Item = UploadFilesRequest> {
  tokio_stream::iter(1..1).map(|i| UploadFilesRequest {
      /* TODO */
      files: vec![
          File{content: vec![]},
          File{content: vec![]},
          File{content: vec![]},
      ],
  })
}
