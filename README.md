# bin-proto

[![Crates.io](https://img.shields.io/crates/v/bin-proto.svg)](https://crates.io/crates/bin-proto)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

[Documentation](https://docs.rs/bin-proto)

Simple bit-level protocol definitions in Rust.

An improved and modernized fork of [protocol](https://crates.io/crates/bin-proto). A more efficient but less feature-rich alternative to [deku](https://crates.io/crates/deku).

This crate adds a trait (and a custom derive for ease-of-use) that can be
implemented on types, allowing structured data to be sent and received from
any binary stream. It is recommended to use [bitstream_io](https://docs.rs/bitstream-io/latest/bitstream_io/) if you need bit streams, as their `BitRead` and `BitWrite` traits are being used internally.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bin-proto = { version = "0.1", features = ["derive"] }
```

And then define a type with the `#[derive(bin_proto::Protocol)]` attribute.

## Alternatives

This crate's main alternative is [deku](https://crates.io/crates/deku). `deku` has more attributes that can be used to customize the co/dec behaviour, but the following is exclusive to `bin-proto`:
- easily pass around arbitrary context
- check that all enum variants fit in bitfield
- co/dec packets to/from a non-byte-aligned stream
- read a stream until it ends, without a length prefix

### Performance comparison

`bin-proto` is significantly faster than `deku` in almost all of the benchmarks. The units for the below table are ns/iter. You can find the benchmarks in the `bench` directory.

|             | Read `enum` | Write `enum` | Read `Vec` | Write `Vec` | Read IPv4 header | Write IPv4 header |
|-------------|-------------|--------------|------------|-------------|------------------|-------------------|
| `bin-proto` | 18          | 86           | 1,615      | 791         | 103              | 126               |
| `deku`      | 32          | 141          | 1,544      | 5,102       | 1,387            | 562               |

## Example

```rust
use bin_proto::Protocol;

#[derive(Debug, Protocol, PartialEq)]
#[protocol(discriminant = "u8")]
#[protocol(bits = 4)]
enum Version {
    V4 = 4,
}

#[derive(Debug, Protocol, PartialEq)]
struct Flags {
    #[protocol(bits = 1)]
    reserved: bool,
    #[protocol(bits = 1)]
    dont_fragment: bool,
    #[protocol(bits = 1)]
    more_fragments: bool,
}

#[derive(Debug, Protocol, PartialEq)]
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

assert_eq!(
    IPv4::from_bytes(&[
            0b0100_0000 // Version: 4
            |    0b0101, // Header Length: 5,
            0x00, // Differentiated Services Codepoint: 0, Explicit Congestion Notification: 0
            0x05, 0x94, // Total Length: 1428
            0x83, 0xf6, // Identification: 0x83f6
            0b0100_0000 // Flags: Don't Fragment
            |  0b0_0000, 0x00, // Fragment Offset: 0
            0x40, // Time to Live: 64
            0x01, // Protocol: 1
            0xeb, 0x6e, // Header Checksum: 0xeb6e
            0x02, 0x01, 0x01, 0x01, // Source Address: 2.1.1.1
            0x02, 0x01, 0x01, 0x02, // Destination Address: 2.1.1.2
        ], &bin_proto::Settings::default()).unwrap(),
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
        header_checksum: 0xeb6e,
        source_address: [2, 1, 1, 1],
        destination_address: [2, 1, 1, 2],
    }
);
```
