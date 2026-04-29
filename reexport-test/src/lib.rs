#![cfg(test)]

use std::marker::PhantomData;

use reexport::{BigEndian, BitCodec, BitDecode, BitEncode};

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
#[bin_proto(discriminant_type = u8)]
#[bin_proto(crate = reexport)]
#[bin_proto(ctx = ())]
pub enum Enum<'a, T: BitDecode + BitEncode> {
    #[bin_proto(discriminant = 1)]
    Variant1 {
        a: T,
        len: u8,
        #[bin_proto(tag = len as usize)]
        arr: Vec<u8>,
    },
    #[bin_proto(discriminant = 2)]
    Variant2(u32, bool, PhantomData<&'a T>),
}

#[test]
fn decode_reexported_enum_variant() {
    assert_eq!(
        (
            Enum::Variant1 {
                a: 64u8,
                len: 2,
                arr: vec![1, 2]
            },
            40
        ),
        Enum::decode_bytes(&[1, 64, 2, 1, 2], BigEndian).unwrap()
    );
}

#[test]
fn encode_reexported_enum_variant() {
    assert_eq!(
        Enum::Variant2::<u32>(20, true, PhantomData)
            .encode_bytes(BigEndian)
            .unwrap(),
        vec![2, 0, 0, 0, 20, 1]
    );
}
