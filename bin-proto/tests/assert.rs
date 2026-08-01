#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{error::ErrorKind, BitDecode, BitDecodeExt, BitEncode, BitEncodeExt};
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
        Assert::decode_bytes::<BigEndian>(&[0x01]).unwrap()
    );
}

#[test]
fn assert_true_encode() {
    assert_eq!(
        vec![0x01],
        Assert { a: 1 }.encode_bytes::<BigEndian>().unwrap()
    );
}

#[test]
fn assert_false_decode() {
    assert_eq!(
        ErrorKind::Assert,
        Assert::decode_bytes::<BigEndian>(&[0x02])
            .unwrap_err()
            .kind()
    );
}

#[test]
fn assert_false_encode() {
    assert_eq!(
        ErrorKind::Assert,
        Assert { a: 2 }
            .encode_bytes::<BigEndian>()
            .unwrap_err()
            .kind()
    );
}
