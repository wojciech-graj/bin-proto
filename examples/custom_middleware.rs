// Custom middleware example.
// All bytes that go through the bin_proto are rotated by an offset of 13.

use std::num::Wrapping;

/// A rot-n middleware.
/// Rotates each byte by a specific offset.
#[derive(Clone, Debug)]
pub struct RotateMiddleware
{
    pub offset: u8,
}

impl RotateMiddleware
{
    pub fn rot13() -> Self {
        RotateMiddleware { offset: 13 }
    }
}

impl bin_proto::wire::Middleware for RotateMiddleware
{
    fn decode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, bin_proto::Error> {
        Ok(data.into_iter().map(|byte| (Wrapping(byte) - Wrapping(self.offset)).0).collect())
    }

    fn encode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, bin_proto::Error> {
        Ok(data.into_iter().map(|byte| (Wrapping(byte) + Wrapping(self.offset)).0).collect())
    }
}

bin_proto::define_middleware_pipeline!(Pipeline {
    rot: RotateMiddleware
});

impl Pipeline
{
    pub fn new() -> Self {
        Pipeline {
            rot: RotateMiddleware::rot13(),
        }
    }
}

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
pub struct Ping {
    id: i64,
    data: Vec<u8>
}

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[protocol(discriminant(u8))]
pub enum Packet {
    #[protocol(discriminant(0))]
    Ping(Ping),
}

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = bin_proto::wire::stream::Connection::new(stream, Pipeline::new(), bin_proto::Settings::default());

    connection.send_packet(&Packet::Ping(Ping { id: 0, data: vec![ 55 ]})).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

