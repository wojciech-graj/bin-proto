#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{BitDecode, BitDecodeExt, BitEncode, BitEncodeExt};
use bitstream_io::BigEndian;

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
struct WithFlexibleArrayMember(#[bin_proto(untagged)] Vec<u8>);

#[test]
fn decode_untagged() {
    assert_eq!(
        WithFlexibleArrayMember::decode_bytes::<BigEndian>(&[1, 2, 3]).unwrap(),
        (WithFlexibleArrayMember(vec![1, 2, 3]), 24)
    );
}

#[test]
fn encodes_untagged() {
    assert_eq!(
        WithFlexibleArrayMember(vec![1, 2, 3])
            .encode_bytes::<BigEndian>()
            .unwrap(),
        vec![1, 2, 3]
    );
}
