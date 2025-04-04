#![cfg(feature = "derive")]

use std::marker::PhantomData;

use bin_proto::{BitCodec, BitDecode, BitEncode};
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
        Foobar {
            a: 3,
            b: '2' as u8,
            c: 1,
        },
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
        BizBong(3, 1, 7),
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
        PartyInTheFront,
        PartyInTheFront::decode_bytes(&[], BigEndian).unwrap()
    );
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
        IPv4Header { version: 4 }
    )
}
