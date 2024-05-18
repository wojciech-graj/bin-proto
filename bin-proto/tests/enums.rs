use std::marker::PhantomData;

use bin_proto::{ByteOrder, Protocol};

#[derive(Debug, bin_proto::Protocol, PartialEq)]
#[protocol(discriminant_type = "u8")]
pub enum Enum<'a, T: Protocol> {
    #[protocol(discriminant = "1")]
    Variant1 {
        a: T,
        len: u8,
        #[protocol(length = "len as usize")]
        arr: Vec<u8>,
    },
    #[protocol(discriminant = "2")]
    Variant2(u32, bool, PhantomData<&'a T>),
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
