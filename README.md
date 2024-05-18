# bin-proto

[![crates](https://img.shields.io/crates/v/bin-proto.svg)](https://crates.io/crates/bin-proto)
[![tests](https://github.com/wojciech-graj/bin-proto/actions/workflows/ci.yml/badge.svg)](https://github.com/wojciech-graj/bin-proto/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/bin-proto/badge.svg)](https://docs.rs/bin-proto)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.txt)

Simple & fast structured bit-level binary co/dec in Rust.

An improved and modernized fork of
[protocol](https://crates.io/crates/protocol). A more efficient but (slightly)
less feature-rich alternative to [deku](https://crates.io/crates/deku).

This crate adds a trait (and a custom derive for ease-of-use) that can be
implemented on types, allowing structured data to be sent and received from any
binary stream. It is recommended to use
[bitstream_io](https://docs.rs/bitstream-io/latest/bitstream_io/) if you need
bit streams, as their `BitRead` and `BitWrite` traits are being used internally.

## Example

Add this to your `Cargo.toml`:

```toml
[dependencies]
bin-proto = "0.4"
```

And then define a type with the `#[derive(bin_proto::Protocol)]` attribute.

```rust
use bin_proto::{Protocol, ProtocolNoCtx};

#[derive(Debug, Protocol, PartialEq)]
#[protocol(discriminant_type = "u8")]
#[protocol(bits = 4)]
enum E {
    V1 = 1,
    #[protocol(discriminant = "4")]
    V4,
}

#[derive(Debug, Protocol, PartialEq)]
struct S {
    #[protocol(bits = 1)]
    bitflag: bool,
    #[protocol(bits = 3)]
    bitfield: u8,
    enum_: E,
    #[protocol(write_value = "self.arr.len() as u8")]
    arr_len: u8,
    #[protocol(length = "arr_len as usize")]
    arr: Vec<u8>,
    #[protocol(flexible_array_member)]
    read_to_end: Vec<u8>,
}

assert_eq!(
    S::from_bytes(&[
        0b1000_0000 // bitflag: true (1)
       | 0b101_0000 // bitfield: 5 (101)
           | 0b0001, // enum_: V1 (0001)
        0x02, // arr_len: 2
        0x21, 0x37, // arr: [0x21, 0x37]
        0x01, 0x02, 0x03, // read_to_end: [0x01, 0x02, 0x03]
    ], bin_proto::ByteOrder::BigEndian).unwrap(),
    S {
        bitflag: true,
        bitfield: 5,
        enum_: E::V1,
        arr_len: 2,
        arr: vec![0x21, 0x37],
        read_to_end: vec![0x01, 0x02, 0x03],
    }
);
```

You can implement `Protocol` on your own types, and parse with context:

```rust
use bin_proto::Protocol;

pub struct Ctx;

pub struct NeedsCtx;

impl Protocol<Ctx> for NeedsCtx {
    fn read(
        _read: &mut dyn bin_proto::BitRead,
        _byte_order: bin_proto::ByteOrder,
        _ctx: &mut Ctx,
    ) -> Result<Self, bin_proto::Error> {
        // Use ctx here
        Ok(Self)
    }

    fn write(
        &self,
        _write: &mut dyn bin_proto::BitWrite,
        _byte_order: bin_proto::ByteOrder,
        _ctx: &mut Ctx,
    ) -> Result<(), bin_proto::Error> {
        // Use ctx here
        Ok(())
    }
}

#[derive(Protocol)]
#[protocol(ctx = "Ctx")]
pub struct WithCtx(NeedsCtx);

WithCtx(NeedsCtx)
    .bytes_ctx(bin_proto::ByteOrder::LittleEndian, &mut Ctx)
    .unwrap();
```

## Performance / Alternatives

This crate's main alternative is [deku](https://crates.io/crates/deku), and [binrw](https://crates.io/crates/binrw) for byte-level protocols.

`bin-proto` is significantly faster than `deku` in all of the tested scenarios.
The units for the below table are `ns/iter`, taken from
[github CI](https://github.com/wojciech-graj/bin-proto/actions/runs/9143081499/job/25139334328).
You can find the benchmarks in the `bench` directory.

|             | Read `enum` | Write `enum` | Read `Vec` | Write `Vec` | Read IPv4 header | Write IPv4 header |
|-------------|-------------|--------------|------------|-------------|------------------|-------------------|
| `bin-proto` | 21          | 93           | 738        | 821         | 151              | 141               |
| `deku`      | 67          | 227          | 3,041      | 9,705       | 2,468            | 821               |

## Roadmap

The following features are planned:
- Bit/byte alignment
- `no_std` support (only after `bitstream_io` supports it)
