#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{BitCodec, BitDecode, BitEncode};
use bitstream_io::BigEndian;

#[derive(Debug, Copy, Clone, BitDecode, BitEncode, PartialEq)]
#[codec(discriminant_type = u8)]
#[codec(bits = 4)]
enum Version {
    V4 = 4,
}

#[derive(Debug, Copy, Clone, BitDecode, BitEncode, PartialEq)]
struct Flags {
    #[codec(bits = 1)]
    reserved: bool,
    #[codec(bits = 1)]
    dont_fragment: bool,
    #[codec(bits = 1)]
    more_fragments: bool,
}

#[derive(Debug, Copy, Clone, BitDecode, BitEncode, PartialEq)]
struct IPv4 {
    version: Version,
    #[codec(bits = 4)]
    internet_header_length: u8,
    #[codec(bits = 6)]
    differentiated_services_code_point: u8,
    #[codec(bits = 2)]
    explicit_congestion_notification: u8,
    total_length: u16,
    identification: u16,
    flags: Flags,
    #[codec(bits = 13)]
    fragment_offset: u16,
    time_to_live: u8,
    protocol: u8,
    header_checksum: u16,
    source_address: [u8; 4],
    destination_address: [u8; 4],
}

#[test]
fn can_encode_decode_ipv4() {
    let raw = [
        0b0100_0000 // Version: 4
            | 0b0101, // Header Length: 5,
        0x00, // Differentiated Services Codepoint: 0, Explicit Congestion Notification: 0
        0x05,
        0x94, // Total Length: 1428
        0x83,
        0xf6, // Identification: 0x83f6
        0b0100_0000 // Flags: Don't Fragment
        | 0b0_0000,
        0x00, // Fragment Offset: 0
        0x40, // Time to Live: 64
        0x01, // Protocol: 1
        0xeb,
        0x6e, // Header Checksum: 0xeb6e
        0x02,
        0x01,
        0x01,
        0x01, // Source Address: 2.1.1.1
        0x02,
        0x01,
        0x01,
        0x02, // Destination Address: 2.1.1.2
    ];
    let parsed = IPv4 {
        version: Version::V4,
        internet_header_length: 5,
        differentiated_services_code_point: 0,
        explicit_congestion_notification: 0,
        total_length: 1428,
        identification: 0x83f6,
        flags: Flags {
            reserved: false,
            dont_fragment: true,
            more_fragments: false,
        },
        fragment_offset: 0x0,
        time_to_live: 64,
        protocol: 1,
        header_checksum: 0xeb6e,
        source_address: [2, 1, 1, 1],
        destination_address: [2, 1, 1, 2],
    };
    assert_eq!((parsed, 160), IPv4::decode_bytes(&raw, BigEndian).unwrap());
    assert_eq!(raw, parsed.encode_bytes(BigEndian).unwrap().as_slice())
}
