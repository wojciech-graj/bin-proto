//! Enable compression.
//! The default middleware pipeline supports compression, but is disabled
//! by default.

use bin_proto::wire::middleware::{self, compression};
use bin_proto::wire::stream;

pub const ALGORITHM: compression::Algorithm = compression::Algorithm::Zlib;

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
pub struct Hello {
    id: i64,
    data: Vec<u8>
}

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = stream::Connection::new(stream, middleware::pipeline::default(), bin_proto::Settings::default());

    connection.middleware.compression = compression::Compression::Enabled(ALGORITHM);

    connection.send_packet(&Hello { id: 0, data: vec![ 55 ]}).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

