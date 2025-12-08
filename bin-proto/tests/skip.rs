#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{BitCodec, BitDecode, BitEncode, Error};
use bitstream_io::BigEndian;

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
struct SkipEncode {
    #[codec(skip_encode)]
    a: u8,
    b: u8,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
struct SkipDecode {
    #[codec(skip_decode)]
    a: u8,
    b: u8,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
struct Skip {
    #[codec(skip)]
    a: u8,
    b: u8,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
#[codec(discriminant_type = u8)]
enum SkipEncodeEnum {
    #[codec(discriminant = 1)]
    A,
    #[codec(discriminant = 2)]
    #[codec(skip_encode)]
    B,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
#[codec(discriminant_type = u8)]
enum SkipDecodeEnum {
    #[codec(discriminant = 1)]
    A,
    #[codec(discriminant = 2)]
    #[codec(skip_decode)]
    #[allow(unused)]
    B,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq, Eq)]
#[codec(discriminant_type = u8)]
enum SkipEnum {
    #[codec(discriminant = 1)]
    A,
    #[codec(discriminant = 2)]
    #[codec(skip)]
    B,
}

#[test]
fn skip_encode_struct() {
    let s = SkipEncode { a: 10, b: 20 };
    let bytes = s.encode_bytes(BigEndian).unwrap();
    assert_eq!(bytes, vec![20]);

    assert!(SkipEncode::decode_bytes(&bytes, BigEndian).is_err());
}

#[test]
fn skip_decode_struct() {
    let s = SkipDecode { a: 10, b: 20 };
    let bytes = s.encode_bytes(BigEndian).unwrap();
    assert_eq!(bytes, vec![10, 20]);

    let (decoded, len) = SkipDecode::decode_bytes(&bytes, BigEndian).unwrap();

    assert_eq!(decoded, SkipDecode { a: 0, b: 10 });
    assert_eq!(len, 8);
}

#[test]
fn skip_struct() {
    let s = Skip { a: 10, b: 20 };
    let bytes = s.encode_bytes(BigEndian).unwrap();
    assert_eq!(bytes, vec![20]);

    let (decoded, len) = Skip::decode_bytes(&bytes, BigEndian).unwrap();

    assert_eq!(decoded, Skip { a: 0, b: 20 });
    assert_eq!(len, 8);
}

#[test]
fn skip_encode_enum() {
    let a = SkipEncodeEnum::A;
    assert_eq!(a.encode_bytes(BigEndian).unwrap(), vec![1]);

    let b = SkipEncodeEnum::B;
    assert!(b.encode_bytes(BigEndian).is_err());
}

#[test]
fn skip_decode_enum() {
    let (decoded, _) = SkipDecodeEnum::decode_bytes(&[1], BigEndian).unwrap();
    assert_eq!(decoded, SkipDecodeEnum::A);

    let result = SkipDecodeEnum::decode_bytes(&[2], BigEndian);
    assert!(matches!(result, Err(Error::Discriminant)));
}

#[test]
fn skip_enum() {
    assert!(SkipEnum::B.encode_bytes(BigEndian).is_err());

    let result = SkipEnum::decode_bytes(&[2], BigEndian);
    assert!(matches!(result, Err(Error::Discriminant)));
}
