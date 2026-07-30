#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{BitCodec, BitDecode, BitEncode, Error};
use bitstream_io::BigEndian;

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
struct Assert {
    #[bin_proto(assert = *a == 1)]
    a: u8,
}

#[test]
fn assert_true_decode() {
    assert_eq!(
        (Assert { a: 1 }, 8),
        Assert::decode_bytes(&[0x01], BigEndian).unwrap()
    );
}

#[test]
fn assert_true_encode() {
    assert_eq!(vec![0x01], Assert { a: 1 }.encode_bytes(BigEndian).unwrap());
}

#[test]
fn assert_false_decode() {
    assert!(matches!(
        Assert::decode_bytes(&[0x02], BigEndian),
        Err(Error::Assert(_))
    ));
}

#[test]
fn assert_false_encode() {
    assert!(matches!(
        Assert { a: 2 }.encode_bytes(BigEndian),
        Err(Error::Assert(_))
    ));
}
