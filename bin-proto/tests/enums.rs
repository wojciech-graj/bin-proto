#![cfg(all(feature = "derive", feature = "alloc"))]

use std::marker::PhantomData;

use bin_proto::{BitCodec, BitDecode, BitEncode};
use bitstream_io::BigEndian;

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
#[codec(discriminant_type = u8)]
#[codec(ctx = ())]
pub enum Enum<'a, T: BitDecode + BitEncode> {
    #[codec(discriminant = 1)]
    Variant1 {
        a: T,
        len: u8,
        #[codec(tag = len as usize)]
        arr: Vec<u8>,
    },
    #[codec(discriminant = 2)]
    Variant2(u32, bool, PhantomData<&'a T>),
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
#[codec(discriminant_type = u8)]
#[codec(bits = 2)]
pub enum Enum2 {
    #[codec(discriminant = 1)]
    Variant1(u8),
    #[codec(discriminant = 2)]
    Variant2(u16),
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
pub struct EnumContainer {
    e: Enum2,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
pub struct TaggedEnumContainer {
    #[codec(tag_type = u16, tag_value = ::bin_proto::Discriminable::discriminant(&self.e) as u16)]
    e: Enum2,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
pub struct BitFieldTaggedEnumContainer {
    #[codec(write_value = ::bin_proto::Discriminable::discriminant(&self.e))]
    #[codec(bits = 3)]
    discriminant: u8,
    #[codec(tag = discriminant)]
    e: Enum2,
}

#[test]
fn decode_enum_variant() {
    assert_eq!(
        Enum::Variant1 {
            a: 64u8,
            len: 2,
            arr: vec![1, 2]
        },
        Enum::decode_bytes(&[1, 64, 2, 1, 2], BigEndian).unwrap()
    );
}

#[test]
fn encode_enum_variant() {
    assert_eq!(
        Enum::Variant2::<u32>(20, true, PhantomData)
            .encode_bytes(BigEndian)
            .unwrap(),
        vec![2, 0, 0, 0, 20, 1]
    );
}

#[test]
fn decode_enum_variant_in_container() {
    assert_eq!(
        EnumContainer {
            e: Enum2::Variant1(2)
        },
        EnumContainer::decode_bytes(&[64, 128], BigEndian).unwrap()
    );
}

#[test]
fn encode_enum_variant_in_container() {
    assert_eq!(
        EnumContainer {
            e: Enum2::Variant2(511)
        }
        .encode_bytes(BigEndian)
        .unwrap(),
        vec![128, 127, 192]
    );
}

#[test]
fn decode_enum_variant_in_container_tagged() {
    assert_eq!(
        TaggedEnumContainer {
            e: Enum2::Variant1(2)
        },
        TaggedEnumContainer::decode_bytes(&[0, 1, 2], BigEndian).unwrap()
    );
}

#[test]
fn encode_enum_variant_in_container_tagged() {
    assert_eq!(
        TaggedEnumContainer {
            e: Enum2::Variant2(511)
        }
        .encode_bytes(BigEndian)
        .unwrap(),
        vec![0, 2, 1, 255,]
    );
}

#[test]
fn decode_enum_variant_in_container_tagged_bitfield() {
    assert_eq!(
        BitFieldTaggedEnumContainer {
            discriminant: 1,
            e: Enum2::Variant1(2)
        },
        BitFieldTaggedEnumContainer::decode_bytes(&[32, 64], BigEndian).unwrap()
    );
}

#[test]
fn encode_enum_variant_in_container_tagged_bitfield() {
    assert_eq!(
        BitFieldTaggedEnumContainer {
            discriminant: 2,
            e: Enum2::Variant2(511)
        }
        .encode_bytes(BigEndian)
        .unwrap(),
        vec![64, 63, 224]
    );
}
