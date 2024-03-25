use protocol::Parcel;
use protocol::Settings;

#[derive(Debug, protocol::Protocol, PartialEq)]
#[protocol(discriminant = "integer")]
#[protocol(bits = 4)]
#[repr(u8)]
enum Version {
    V4 = 4,
}

#[derive(Debug, protocol::Protocol, PartialEq)]
struct Flags {
    #[protocol(bits = 1)]
    reserved: bool,
    #[protocol(bits = 1)]
    dont_fragment: bool,
    #[protocol(bits = 1)]
    more_fragments: bool,
}

#[derive(Debug, protocol::Protocol, PartialEq)]
struct IPv4 {
    version: Version,
    #[protocol(bits = 4)]
    internet_header_length: u8,
    #[protocol(bits = 6)]
    differentiated_services_code_point: u8,
    #[protocol(bits = 2)]
    explicit_congestion_notification: u8,
    total_length: u16,
    identification: u16,
    flags: Flags,
    #[protocol(bits = 13)]
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
        0x45, 0x00, 0x05, 0x94, 0x83, 0xf6, 0x40, 0x00, 0x40, 0x01, 0xeb, 0x6e, 0x02, 0x01, 0x01,
        0x01, 0x02, 0x01, 0x01, 0x02,
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
    assert_eq!(
        parsed,
        IPv4::from_raw_bytes(&raw, &Settings::default()).unwrap()
    );
    assert_eq!(
        raw,
        parsed.raw_bytes(&Settings::default()).unwrap().as_slice()
    )
}
