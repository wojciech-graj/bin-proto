#![cfg(feature = "derive")]

use bin_proto::{ByteOrder, ProtocolNoCtx, ProtocolRead, ProtocolWrite};

#[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
struct WithFlexibleArrayMember(#[protocol(flexible_array_member)] Vec<u8>);

#[test]
fn reads_flexible_array_member() {
    assert_eq!(
        WithFlexibleArrayMember::from_bytes(&[1, 2, 3], ByteOrder::BigEndian).unwrap(),
        WithFlexibleArrayMember(vec![1, 2, 3])
    );
}

#[test]
fn writes_flexible_array_member() {
    assert_eq!(
        WithFlexibleArrayMember(vec![1, 2, 3])
            .bytes(ByteOrder::BigEndian)
            .unwrap(),
        vec![1, 2, 3]
    );
}
