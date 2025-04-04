#![cfg(feature = "derive")]

use bin_proto::{BitCodec, BitDecode, BitEncode};
use bitstream_io::BigEndian;

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
struct WithFlexibleArrayMember(#[codec(flexible_array_member)] Vec<u8>);

#[test]
fn decode_flexible_array_member() {
    assert_eq!(
        WithFlexibleArrayMember::decode_bytes(&[1, 2, 3], BigEndian).unwrap(),
        WithFlexibleArrayMember(vec![1, 2, 3])
    );
}

#[test]
fn encodes_flexible_array_member() {
    assert_eq!(
        WithFlexibleArrayMember(vec![1, 2, 3])
            .encode_bytes(BigEndian)
            .unwrap(),
        vec![1, 2, 3]
    );
}
