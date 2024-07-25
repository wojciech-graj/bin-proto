use std::marker::PhantomData;

use bin_proto::{ByteOrder, ProtocolNoCtx, ProtocolRead, ProtocolWrite};

#[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
#[protocol(discriminant_type = "u8")]
#[protocol(ctx = "()")]
pub enum Enum<'a, T: ProtocolRead + ProtocolWrite> {
    #[protocol(discriminant = "1")]
    Variant1 {
        a: T,
        len: u8,
        #[protocol(tag = "len as usize")]
        arr: Vec<u8>,
    },
    #[protocol(discriminant = "2")]
    Variant2(u32, bool, PhantomData<&'a T>),
}

#[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
#[protocol(discriminant_type = "u8")]
#[protocol(bits = 2)]
pub enum Enum2 {
    #[protocol(discriminant = "1")]
    Variant1(u8),
    #[protocol(discriminant = "2")]
    Variant2(u16),
}

#[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
pub struct EnumContainer {
    e: Enum2,
}

#[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
pub struct TaggedEnumContainer {
    #[protocol(tag(
        type = "u16",
        write_value = "::bin_proto::Discriminable::discriminant(&self.e) as u16"
    ))]
    e: Enum2,
}

#[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
pub struct BitFieldTaggedEnumContainer {
    #[protocol(write_value = "::bin_proto::Discriminable::discriminant(&self.e)")]
    #[protocol(bits = 3)]
    discriminant: u8,
    #[protocol(tag = "discriminant")]
    e: Enum2,
}

#[test]
fn read_enum_variant() {
    assert_eq!(
        Enum::Variant1 {
            a: 64u8,
            len: 2,
            arr: vec![1, 2]
        },
        Enum::from_bytes(&[1, 64, 2, 1, 2], ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn write_enum_variant() {
    assert_eq!(
        Enum::Variant2::<u32>(20, true, PhantomData)
            .bytes(ByteOrder::BigEndian)
            .unwrap(),
        vec![2, 0, 0, 0, 20, 1]
    );
}

#[test]
fn read_enum_variant_in_container() {
    assert_eq!(
        EnumContainer {
            e: Enum2::Variant1(2)
        },
        EnumContainer::from_bytes(&[64, 128], ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn write_enum_variant_in_container() {
    assert_eq!(
        EnumContainer {
            e: Enum2::Variant2(511)
        }
        .bytes(ByteOrder::BigEndian)
        .unwrap(),
        vec![128, 127, 192]
    );
}

#[test]
fn read_enum_variant_in_container_tagged() {
    assert_eq!(
        TaggedEnumContainer {
            e: Enum2::Variant1(2)
        },
        TaggedEnumContainer::from_bytes(&[0, 1, 2], ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn write_enum_variant_in_container_tagged() {
    assert_eq!(
        TaggedEnumContainer {
            e: Enum2::Variant2(511)
        }
        .bytes(ByteOrder::BigEndian)
        .unwrap(),
        vec![0, 2, 1, 255,]
    );
}

#[test]
fn read_enum_variant_in_container_tagged_bitfield() {
    assert_eq!(
        BitFieldTaggedEnumContainer {
            discriminant: 1,
            e: Enum2::Variant1(2)
        },
        BitFieldTaggedEnumContainer::from_bytes(&[32, 64], ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn write_enum_variant_in_container_tagged_bitfield() {
    assert_eq!(
        BitFieldTaggedEnumContainer {
            discriminant: 2,
            e: Enum2::Variant2(511)
        }
        .bytes(ByteOrder::BigEndian)
        .unwrap(),
        vec![64, 63, 224]
    );
}
