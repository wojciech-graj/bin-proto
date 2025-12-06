#![cfg(all(feature = "derive", feature = "alloc"))]

use std::marker::PhantomData;

use bin_proto::{BitCodec, BitDecode, BitEncode, Error};
use bitstream_io::BigEndian;

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct Foobar {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct BizBong(u8, u8, pub u8);

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct PartyInTheFront;

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[codec(ctx = ())]
pub struct NamedFieldsWithGenerics<A: BitDecode + BitEncode, D: BitDecode + BitEncode> {
    pub value: A,
    pub del: D,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[codec(ctx = Ctx)]
pub struct UnnamedFieldsWithGenerics<
    Ctx,
    A: BitDecode<Ctx> + BitEncode<Ctx>,
    D: BitDecode<Ctx> + BitEncode<Ctx>,
>(A, D, PhantomData<Ctx>);

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[codec(ctx = ())]
pub struct StructWithExistingBoundedGenerics<
    A: ::std::fmt::Display + ::std::fmt::Debug + BitDecode + BitEncode,
> {
    foo: A,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct WithDefault {
    a: u8,
    #[codec(skip_decode)]
    b: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[codec(pad_before = 1, pad_after = 12)]
pub struct Padded {
    a: u8,
    #[codec(pad_before = 4, pad_after = 8)]
    b: u8,
    c: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[codec(magic = &[0x09u8])]
pub struct Magic {
    a: u8,
    #[codec(magic = b"\x01\x02\x03")]
    b: u8,
}

#[test]
fn named_fields_are_correctly_written() {
    assert_eq!(
        vec![3, '2' as u8, 1],
        Foobar {
            a: 3,
            b: '2' as u8,
            c: 1,
        }
        .encode_bytes(BigEndian)
        .unwrap()
    );
}

#[test]
fn named_fields_are_correctly_decoded() {
    assert_eq!(
        (
            Foobar {
                a: 3,
                b: '2' as u8,
                c: 1,
            },
            24
        ),
        Foobar::decode_bytes(&[3, '2' as u8, 1], BigEndian).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_written() {
    assert_eq!(
        vec![6, 1, 9],
        BizBong(6, 1, 9).encode_bytes(BigEndian).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_decoded() {
    assert_eq!(
        (BizBong(3, 1, 7), 24),
        BizBong::decode_bytes(&[3, 1, 7], BigEndian).unwrap()
    );
}

#[test]
fn unit_structs_are_correctly_written() {
    assert_eq!(PartyInTheFront.encode_bytes(BigEndian).unwrap(), &[]);
}

#[test]
fn unit_structs_are_correctly_decoded() {
    assert_eq!(
        (PartyInTheFront, 0),
        PartyInTheFront::decode_bytes(&[], BigEndian).unwrap()
    );
}

#[test]
fn default_written_correctly() {
    assert_eq!(
        vec![1, 2],
        WithDefault { a: 1, b: 2 }.encode_bytes(BigEndian).unwrap()
    )
}

#[test]
fn default_read_correctly() {
    assert_eq!(
        (
            WithDefault {
                a: 1,
                b: Default::default()
            },
            8
        ),
        WithDefault::decode_bytes(&[1], BigEndian).unwrap()
    )
}

#[test]
fn pad_written_correctly() {
    assert_eq!(
        vec![0, 128, 16, 0, 24, 0, 0],
        Padded { a: 1, b: 2, c: 3 }.encode_bytes(BigEndian).unwrap()
    )
}

#[test]
fn pad_read_correctly() {
    assert_eq!(
        (Padded { a: 1, b: 2, c: 3 }, 49),
        Padded::decode_bytes(&[0, 128, 16, 0, 24, 0, 0], BigEndian).unwrap()
    )
}

#[test]
fn magic_written_correctly() {
    assert_eq!(
        vec![9, 4, 1, 2, 3, 5],
        Magic { a: 4, b: 5 }.encode_bytes(BigEndian).unwrap()
    )
}

#[test]
fn magic_read_correctly() {
    assert_eq!(
        (Magic { a: 4, b: 5 }, 48),
        Magic::decode_bytes(&[9, 4, 1, 2, 3, 5], BigEndian).unwrap()
    )
}

#[test]
fn incorrect_magic_fails() {
    assert!(matches!(
        &Magic::decode_bytes(&[10, 4, 1, 2, 3, 5], BigEndian),
        Err(Error::Magic(&[9]))
    ));
}

#[test]
fn ipv4() {
    #[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
    struct IPv4Header {
        #[codec(bits = 4)]
        version: u8,
    }

    assert_eq!(
        IPv4Header::decode_bytes(&[0x45], BigEndian).unwrap(),
        (IPv4Header { version: 4 }, 4)
    )
}
