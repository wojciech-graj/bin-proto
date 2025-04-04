#![cfg(feature = "derive")]

use bin_proto::{BitCodec, BitDecode, BitEncode};
use bitstream_io::BigEndian;

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub reason_length: u8,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct WithElementsLength {
    pub count: u32,
    pub foo: bool,
    #[codec(tag = count as usize)]
    pub data: Vec<u32>,
}

#[derive(BitDecode, Debug, PartialEq, Eq)]
pub struct OptionalWriteValue {
    #[codec(tag_type = u8)]
    pub data: Vec<u32>,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct WithElementsLengthAuto {
    #[codec(write_value = self.data.len() as u32)]
    pub count: u32,
    pub foo: bool,
    #[codec(tag = count as usize)]
    pub data: Vec<u32>,
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
#[codec(discriminant_type = u8)]
pub enum WithElementsLengthAutoEnum {
    #[codec(discriminant = 1)]
    Variant {
        #[codec(write_value = data.len() as u32)]
        count: u32,
        foo: bool,
        #[codec(tag = count as usize)]
        data: Vec<u32>,
    },
}

#[derive(BitDecode, BitEncode, Debug, PartialEq, Eq)]
pub struct Prepended {
    #[codec(tag_type = u32, tag_value = self.data.len() as u32)]
    pub data: Vec<u32>,
}

#[test]
fn can_decode_length_prefix_3_elements() {
    assert_eq!(
        WithElementsLength {
            count: 3,
            foo: true,
            data: vec![1, 2, 3],
        },
        WithElementsLength::decode_bytes(
            &[
                0, 0, 0, 3, // disjoint length prefix
                1, // boolean true
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            BigEndian,
        )
        .unwrap()
    );
}

#[test]
fn can_encode_auto_length_prefix_3_elements_enum() {
    assert_eq!(
        WithElementsLengthAuto {
            count: 0,
            foo: true,
            data: vec![1, 2, 3],
        }
        .encode_bytes(BigEndian)
        .unwrap(),
        vec![
            0, 0, 0, 3, // disjoint length prefix
            1, // boolean true
            0, 0, 0, 1, // 1
            0, 0, 0, 2, // 2
            0, 0, 0, 3 // 3
        ],
    );
}

#[test]
fn can_decode_length_prefix_3_elements_enum() {
    assert_eq!(
        WithElementsLengthAutoEnum::Variant {
            count: 3,
            foo: true,
            data: vec![1, 2, 3],
        },
        WithElementsLengthAutoEnum::decode_bytes(
            &[
                1, // Discriminant
                0, 0, 0, 3, // disjoint length prefix
                1, // boolean true
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            BigEndian,
        )
        .unwrap()
    );
}

#[test]
fn can_encode_auto_length_prefix_3_elements() {
    assert_eq!(
        WithElementsLengthAutoEnum::Variant {
            count: 0,
            foo: true,
            data: vec![1, 2, 3],
        }
        .encode_bytes(BigEndian)
        .unwrap(),
        vec![
            1, // Discriminant
            0, 0, 0, 3, // disjoint length prefix
            1, // boolean true
            0, 0, 0, 1, // 1
            0, 0, 0, 2, // 2
            0, 0, 0, 3 // 3
        ],
    );
}

#[test]
fn can_decode_prepended_length_prefix_3_elements() {
    assert_eq!(
        Prepended {
            data: vec![1, 2, 3],
        },
        Prepended::decode_bytes(
            &[
                0, 0, 0, 3, // length prefix
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            BigEndian,
        )
        .unwrap()
    );
}

#[test]
fn can_encode_prepended_length_prefix_3_elements() {
    assert_eq!(
        Prepended {
            data: vec![1, 2, 3],
        }
        .encode_bytes(BigEndian)
        .unwrap(),
        vec![
            0, 0, 0, 3, // disjoint length prefix
            0, 0, 0, 1, // 1
            0, 0, 0, 2, // 2
            0, 0, 0, 3 // 3
        ],
    );
}
