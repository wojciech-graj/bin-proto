#![cfg(all(feature = "derive", feature = "alloc"))]

use core::marker::PhantomData;

use bin_proto::{error::ErrorKind, BitDecode, BitDecodeExt, BitEncode, BitEncodeExt};
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
#[bin_proto(ctx = ())]
pub struct NamedFieldsWithGenerics<A, D> {
    pub value: A,
    pub del: D,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[bin_proto(ctx = Ctx)]
pub struct UnnamedFieldsWithGenerics<Ctx, A, D>(A, D, PhantomData<Ctx>);

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[bin_proto(ctx = ())]
pub struct StructWithExistingBoundedGenerics<A: ::core::fmt::Display + ::core::fmt::Debug> {
    foo: A,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct GenericStructNoExplicitCtx<T> {
    pub field: T,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct GenericStructPreBounded<T> {
    pub field: T,
}

// Deprecated
#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct GenericStructNestedFieldSelf<T> {
    pub len: u8,
    #[bin_proto(tag_type = u8, tag_value = self.items.len() as u8)]
    pub items: Vec<T>,
    pub boxed: Box<T>,
    pub marker: PhantomData<T>,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct GenericStructNestedField<T> {
    pub len: u8,
    #[bin_proto(tag_type = u8, tag_value = items.len() as u8)]
    pub items: Vec<T>,
    pub boxed: Box<T>,
    pub marker: PhantomData<T>,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct GenericStructSkippedField<T, U> {
    pub used: T,
    #[bin_proto(skip)]
    pub unused: Option<U>,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct GenericStructBitField<T> {
    #[bin_proto(bits = 4)]
    pub field: T,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct WithDefault {
    a: u8,
    #[bin_proto(skip_decode)]
    b: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[bin_proto(pad_before = 1, pad_after = 12)]
pub struct Padded {
    a: u8,
    #[bin_proto(pad_before = 4, pad_after = 8)]
    b: u8,
    c: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[bin_proto(magic = &[0x09u8])]
pub struct Magic {
    a: u8,
    #[bin_proto(magic = b"\x01\x02\x03")]
    b: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[bin_proto(magic = &[0x11])]
pub struct Magic2;

pub trait Gat {
    type Assoc<'a>: Copy;
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct WithGat<T>
where
    T: Gat,
{
    gat: T::Assoc<'static>,
    _phantom: PhantomData<T>,
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
        .encode_bytes::<BigEndian>()
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
        Foobar::decode_bytes::<BigEndian>(&[3, '2' as u8, 1]).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_written() {
    assert_eq!(
        vec![6, 1, 9],
        BizBong(6, 1, 9).encode_bytes::<BigEndian>().unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_decoded() {
    assert_eq!(
        (BizBong(3, 1, 7), 24),
        BizBong::decode_bytes::<BigEndian>(&[3, 1, 7]).unwrap()
    );
}

#[test]
fn unit_structs_are_correctly_written() {
    assert_eq!(PartyInTheFront.encode_bytes::<BigEndian>().unwrap(), &[]);
}

#[test]
fn unit_structs_are_correctly_decoded() {
    assert_eq!(
        (PartyInTheFront, 0),
        PartyInTheFront::decode_bytes::<BigEndian>(&[]).unwrap()
    );
}

#[test]
fn default_written_correctly() {
    assert_eq!(
        vec![1, 2],
        WithDefault { a: 1, b: 2 }
            .encode_bytes::<BigEndian>()
            .unwrap()
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
        WithDefault::decode_bytes::<BigEndian>(&[1]).unwrap()
    )
}

#[test]
fn pad_written_correctly() {
    assert_eq!(
        vec![0, 128, 16, 0, 24, 0, 0],
        Padded { a: 1, b: 2, c: 3 }
            .encode_bytes::<BigEndian>()
            .unwrap()
    )
}

#[test]
fn pad_read_correctly() {
    assert_eq!(
        (Padded { a: 1, b: 2, c: 3 }, 49),
        Padded::decode_bytes::<BigEndian>(&[0, 128, 16, 0, 24, 0, 0]).unwrap()
    )
}

#[test]
fn magic_written_correctly() {
    assert_eq!(
        vec![9, 4, 1, 2, 3, 5],
        Magic { a: 4, b: 5 }.encode_bytes::<BigEndian>().unwrap()
    )
}

#[test]
fn magic_read_correctly() {
    assert_eq!(
        (Magic { a: 4, b: 5 }, 48),
        Magic::decode_bytes::<BigEndian>(&[9, 4, 1, 2, 3, 5]).unwrap()
    )
}

#[test]
fn incorrect_magic_fails() {
    assert_eq!(
        ErrorKind::Magic,
        Magic::decode_bytes::<BigEndian>(&[10, 4, 1, 2, 3, 5])
            .unwrap_err()
            .kind()
    );
}

#[test]
fn magic_unit_written_correctly() {
    assert_eq!(vec![0x11], Magic2.encode_bytes::<BigEndian>().unwrap())
}

#[test]
fn magic_unit_read_correctly() {
    assert_eq!(
        (Magic2, 8),
        Magic2::decode_bytes::<BigEndian>(&[0x11]).unwrap()
    )
}

#[test]
fn incorrect_magic_unit_fails() {
    assert_eq!(
        ErrorKind::Magic,
        Magic2::decode_bytes::<BigEndian>(&[0x12])
            .unwrap_err()
            .kind()
    );
}

#[test]
fn generic_struct_roundtrips() {
    let value = GenericStructNoExplicitCtx { field: 0x42u16 };
    let bytes = value.encode_bytes::<BigEndian>().unwrap();
    assert_eq!(vec![0x00, 0x42], bytes);
    assert_eq!(
        (value, 16),
        GenericStructNoExplicitCtx::decode_bytes::<BigEndian>(&bytes).unwrap()
    );
}

#[test]
fn generic_struct_nested_field_roundtrips() {
    let value = GenericStructNestedField {
        len: 3,
        items: vec![1u8, 2, 3],
        boxed: Box::new(7u8),
        marker: PhantomData,
    };
    let bytes = value.encode_bytes::<BigEndian>().unwrap();
    assert_eq!(vec![3, 3, 1, 2, 3, 7], bytes);
    assert_eq!(
        (value, 48),
        GenericStructNestedField::decode_bytes::<BigEndian>(&bytes).unwrap()
    );
}

#[test]
fn generic_struct_skipped_field_roundtrips() {
    // `U` must not require codec bounds because the field is skipped.
    struct NoCodec;
    let value = GenericStructSkippedField::<u8, NoCodec> {
        used: 5,
        unused: None,
    };
    let bytes = value.encode_bytes::<BigEndian>().unwrap();
    assert_eq!(vec![5], bytes);
}

#[test]
fn generic_struct_bitfield_roundtrips() {
    let value = GenericStructBitField { field: 0x0Fu8 };
    let bytes = value.encode_bytes::<BigEndian>().unwrap();
    assert_eq!(vec![0xF0], bytes);
    assert_eq!(
        (value, 4),
        GenericStructBitField::decode_bytes::<BigEndian>(&bytes).unwrap()
    );
}

#[test]
fn ipv4() {
    #[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
    struct IPv4Header {
        #[bin_proto(bits = 4)]
        version: u8,
    }

    assert_eq!(
        IPv4Header::decode_bytes::<BigEndian>(&[0x45]).unwrap(),
        (IPv4Header { version: 4 }, 4)
    )
}
