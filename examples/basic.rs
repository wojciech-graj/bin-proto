#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
pub struct Handshake;

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
pub struct Hello {
    id: i64,
    data: Vec<u8>,
}

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
pub struct Goodbye {
    id: i64,
    reason: String,
}

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    enabled: bool
}

// Defines a packet kind enum.
#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
pub enum Packet {
    #[protocol(discriminant(0x00))]
    Handshake(Handshake),
    #[protocol(discriminant(0x01))]
    Hello(Hello),
    #[protocol(discriminant(0x02))]
    Goodbye(Goodbye),
}

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let settings = bin_proto::Settings {
        byte_order: bin_proto::ByteOrder::LittleEndian,
        ..Default::default()
    };
    let mut connection = bin_proto::wire::stream::Connection::new(stream, bin_proto::wire::middleware::pipeline::default(), settings);

    connection.send_packet(&Packet::Handshake(Handshake)).unwrap();
    connection.send_packet(&Packet::Hello(Hello { id: 0, data: vec![ 55 ]})).unwrap();
    connection.send_packet(&Packet::Goodbye(Goodbye { id: 0, reason: "leaving".to_string() })).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

