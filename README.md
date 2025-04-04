# bin-proto

[![crates](https://img.shields.io/crates/v/bin-proto.svg)](https://crates.io/crates/bin-proto)
[![tests](https://github.com/wojciech-graj/bin-proto/actions/workflows/ci.yml/badge.svg)](https://github.com/wojciech-graj/bin-proto/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/bin-proto/badge.svg)](https://docs.rs/bin-proto)
![msrv](https://img.shields.io/crates/msrv/bin-proto)
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
bin-proto = "0.7"
```

And then define a type with the `#[derive(bin_proto::BitDecode, bin_proto::BitEncode)]` attributes.

```rust
use bin_proto::{BitDecode, BitEncode, BitCodec};

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
#[codec(discriminant_type = u8)]
#[codec(bits = 4)]
enum E {
    V1 = 1,
    #[codec(discriminant = 4)]
    V4,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
struct S {
    #[codec(bits = 1)]
    bitflag: bool,
    #[codec(bits = 3)]
    bitfield: u8,
    enum_: E,
    #[codec(write_value = self.arr.len() as u8)]
    arr_len: u8,
    #[codec(tag = arr_len as usize)]
    arr: Vec<u8>,
    #[codec(tag_type = u16, tag_value = self.prefixed_arr.len() as u16)]
    prefixed_arr: Vec<u8>,
    #[codec(flexible_array_member)]
    read_to_end: Vec<u8>,
}

assert_eq!(
    S::decode_bytes(&[
        0b1000_0000 // bitflag: true (1)
       | 0b101_0000 // bitfield: 5 (101)
           | 0b0001, // enum_: V1 (0001)
        0x02, // arr_len: 2
        0x21, 0x37, // arr: [0x21, 0x37]
        0x00, 0x01, 0x33, // prefixed_arr: [0x33]
        0x01, 0x02, 0x03, // read_to_end: [0x01, 0x02, 0x03]
    ], bin_proto::BigEndian).unwrap(),
    S {
        bitflag: true,
        bitfield: 5,
        enum_: E::V1,
        arr_len: 2,
        arr: vec![0x21, 0x37],
        prefixed_arr: vec![0x33],
        read_to_end: vec![0x01, 0x02, 0x03],
    }
);
```

You can implement `BitEncode` and `BitDecode` on your own types, and parse with context:

```rust
use bin_proto::{BitDecode, BitEncode};

pub struct Ctx;

pub struct NeedsCtx;

impl BitDecode<Ctx> for NeedsCtx {
    fn decode<R, E>(
        _read: &mut R,
        _ctx: &mut Ctx,
        _tag: (),
    ) -> bin_proto::Result<Self>
    where
        R: bin_proto::BitRead,
        E: bin_proto::Endianness,
    {
        // Use ctx here
        Ok(Self)
    }
}

impl BitEncode<Ctx> for NeedsCtx {
    fn encode<W, E>(
        &self,
        _write: &mut W,
        _ctx: &mut Ctx,
        _tag: (),
    ) -> bin_proto::Result<()>
    where
        W: bin_proto::BitWrite,
        E: bin_proto::Endianness,
    {
        // Use ctx here
        Ok(())
    }
}

#[derive(BitDecode, BitEncode)]
#[codec(ctx = Ctx)]
pub struct WithCtx(NeedsCtx);

WithCtx(NeedsCtx)
    .encode_bytes_ctx(bin_proto::BigEndian, &mut Ctx, ())
    .unwrap();
```

## Performance / Alternatives

This crate's main alternative is [deku](https://crates.io/crates/deku), and [binrw](https://crates.io/crates/binrw) for byte-level protocols.

`bin-proto` is significantly faster than `deku` in most of the tested scenarios.
The units for the below table are `ns`, taken from
[github CI](https://github.com/wojciech-graj/bin-proto/actions/runs/14273083815/job/40010524980).
You can find the benchmarks in the `bench` directory.

|             | Read `enum` | Write `enum` | Read `Vec` | Write `Vec` | Read IPv4 header | Write IPv4 header |
|-------------|-------------|--------------|------------|-------------|------------------|-------------------|
| `bin-proto` | 20          | 65           | 1,052      | 588         | 134              | 122               |
| `deku`      | 1           | 96           | 899        | 1,070       | 4,086            | 703               |
