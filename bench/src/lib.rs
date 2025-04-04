#![feature(test)]
extern crate test;

mod vec {
    use test::{black_box, Bencher};

    mod bench_bin_proto {
        use super::*;
        use bin_proto::{BitCodec, BitDecode, BitEncode};

        #[derive(Debug, BitDecode, BitEncode, PartialEq)]
        struct V {
            #[codec(write_value = self.data.len() as u8)]
            count: u8,
            #[codec(tag = count as usize)]
            data: Vec<u8>,
        }

        #[bench]
        fn bench_write(b: &mut Bencher) {
            b.iter(|| {
                black_box(
                    V {
                        count: 255,
                        data: (0..255).collect(),
                    }
                    .encode_bytes(bin_proto::BigEndian),
                )
                .unwrap();
            });
        }

        #[bench]
        fn bench_read(b: &mut Bencher) {
            let mut v = vec![255u8];
            v.extend((0..255).collect::<Vec<_>>());
            b.iter(|| {
                black_box(V::decode_bytes(v.as_slice(), bin_proto::BigEndian)).unwrap();
            })
        }
    }

    mod bench_deku {
        use super::*;
        use deku::prelude::*;

        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        struct V {
            #[deku(update = "self.data.len()")]
            count: u8,
            #[deku(count = "count")]
            data: Vec<u8>,
        }

        #[bench]
        fn bench_write(b: &mut Bencher) {
            b.iter(|| {
                black_box(
                    V {
                        count: 255,
                        data: (0..255).collect(),
                    }
                    .to_bytes(),
                )
                .unwrap();
            });
        }

        #[bench]
        fn bench_read(b: &mut Bencher) {
            let mut v = vec![255u8];
            v.extend((0..255).collect::<Vec<_>>());
            b.iter(|| {
                black_box(V::from_bytes((v.as_slice(), 0))).unwrap();
            });
        }
    }
}

mod enum_ {
    use test::{black_box, Bencher};

    mod bench_bin_proto {
        use super::*;
        use bin_proto::{BitCodec, BitDecode, BitEncode};

        #[derive(Debug, BitDecode, BitEncode, PartialEq)]
        #[codec(discriminant_type = u8)]
        enum E {
            V0 = 0,
            V1 = 1,
            V2 = 2,
            V3 = 3,
        }

        #[bench]
        fn bench_enum_write(b: &mut Bencher) {
            b.iter(|| {
                black_box({
                    E::V0.encode_bytes(bin_proto::BigEndian).unwrap();
                    E::V1.encode_bytes(bin_proto::BigEndian).unwrap();
                    E::V2.encode_bytes(bin_proto::BigEndian).unwrap();
                    E::V3.encode_bytes(bin_proto::BigEndian).unwrap();
                })
            });
        }

        #[bench]
        fn bench_enum_read(b: &mut Bencher) {
            b.iter(|| {
                black_box(for i in 0..4 {
                    E::decode_bytes(&[i], bin_proto::BigEndian).unwrap();
                })
            });
        }
    }

    mod bench_deku {
        use super::*;
        use deku::prelude::*;

        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        #[deku(id_type = "u8")]
        enum E {
            #[deku(id = "0")]
            V0,
            #[deku(id = "1")]
            V1,
            #[deku(id = "2")]
            V2,
            #[deku(id = "3")]
            V3,
        }

        #[bench]
        fn bench_write(b: &mut Bencher) {
            b.iter(|| {
                black_box({
                    E::V0.to_bytes().unwrap();
                    E::V1.to_bytes().unwrap();
                    E::V2.to_bytes().unwrap();
                    E::V3.to_bytes().unwrap();
                })
            });
        }

        #[bench]
        fn bench_read(b: &mut Bencher) {
            b.iter(|| {
                black_box(for i in 0..4 {
                    E::from_bytes((&[i], 0)).unwrap();
                })
            });
        }
    }
}

mod ipv4 {
    use std::net::Ipv4Addr;
    use test::{black_box, Bencher};

    mod bench_bin_proto {
        use super::*;
        use bin_proto::{BitCodec, BitDecode, BitEncode};

        #[derive(Debug, BitDecode, BitEncode, PartialEq)]
        #[codec(discriminant_type = u8)]
        #[codec(bits = 4)]
        enum Version {
            V4 = 4,
        }

        #[derive(Debug, BitDecode, BitEncode, PartialEq)]
        struct Flags {
            #[codec(bits = 1)]
            reserved: bool,
            #[codec(bits = 1)]
            dont_fragment: bool,
            #[codec(bits = 1)]
            more_fragments: bool,
        }

        #[derive(Debug, BitDecode, BitEncode, PartialEq)]
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
            source_address: Ipv4Addr,
            destination_address: Ipv4Addr,
        }

        #[bench]
        fn bench_ipv4_write(b: &mut Bencher) {
            b.iter(|| {
                black_box(
                    IPv4 {
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
                        header_checksum: 0xeeee,
                        source_address: Ipv4Addr::new(2, 1, 1, 1),
                        destination_address: Ipv4Addr::new(2, 1, 1, 2),
                    }
                    .encode_bytes(bin_proto::BigEndian),
                )
                .unwrap();
            });
        }

        #[bench]
        fn bench_ipv4_read(b: &mut Bencher) {
            b.iter(|| {
                black_box(IPv4::decode_bytes(
                    &[
                        0b0100_0000 // Version: 4
            |    0b0101, // Header Length: 5,
                        0x00, // Differentiated Services Codepoint: 0, Explicit Congestion Notification: 0
                        0x05,
                        0x94, // Total Length: 1428
                        0x83,
                        0xf6, // Identification: 0x83f6
                        0b0100_0000 // Flags: Don't Fragment
            |  0b0_0000,
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
                    ],
                    bin_proto::BigEndian,
                ))
                .unwrap();
            });
        }
    }

    mod bench_deku {
        use super::*;
        use deku::prelude::*;

        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        #[deku(id_type = "u8")]
        #[deku(bits = 4)]
        enum Version {
            #[deku(id = "4")]
            V4,
        }

        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        struct Flags {
            #[deku(bits = 1)]
            reserved: bool,
            #[deku(bits = 1)]
            dont_fragment: bool,
            #[deku(bits = 1)]
            more_fragments: bool,
        }

        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        struct IPv4 {
            version: Version,
            #[deku(bits = 4)]
            internet_header_length: u8,
            #[deku(bits = 6)]
            differentiated_services_code_point: u8,
            #[deku(bits = 2)]
            explicit_congestion_notification: u8,
            total_length: u16,
            identification: u16,
            flags: Flags,
            #[deku(bits = 13)]
            fragment_offset: u16,
            time_to_live: u8,
            protocol: u8,
            header_checksum: u16,
            source_address: Ipv4Addr,
            destination_address: Ipv4Addr,
        }

        #[bench]
        fn bench_ipv4_write_deku(b: &mut Bencher) {
            b.iter(|| {
                black_box(
                    IPv4 {
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
                        header_checksum: 0xeeee,
                        source_address: Ipv4Addr::new(2, 1, 1, 1),
                        destination_address: Ipv4Addr::new(2, 1, 1, 2),
                    }
                    .to_bytes(),
                )
                .unwrap();
            });
        }

        #[bench]
        fn bench_ipv4_read_deku(b: &mut Bencher) {
            b.iter(|| {
                black_box(IPv4::from_bytes((
                    &[
                        0b0100_0000 // Version: 4
            |    0b0101, // Header Length: 5,
                        0x00, // Differentiated Services Codepoint: 0, Explicit Congestion Notification: 0
                        0x05,
                        0x94, // Total Length: 1428
                        0x83,
                        0xf6, // Identification: 0x83f6
                        0b0100_0000 // Flags: Don't Fragment
            |  0b0_0000,
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
                    ],
                    0,
                )))
                .unwrap();
            });
        }
    }
}
