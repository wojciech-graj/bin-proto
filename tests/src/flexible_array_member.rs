use bin_proto::Protocol;
use bin_proto::Settings;

#[derive(Debug, bin_proto::Protocol, PartialEq)]
struct WithFlexibleArrayMember(#[protocol(flexible_array_member)] Vec<u8>);

#[test]
fn reads_flexible_array_member() {
    assert_eq!(
        WithFlexibleArrayMember::from_bytes(&[1, 2, 3], &Settings::default()).unwrap(),
        WithFlexibleArrayMember(vec![1, 2, 3])
    );
}

#[test]
fn writes_flexible_array_member() {
    assert_eq!(
        WithFlexibleArrayMember(vec![1, 2, 3])
            .bytes(&Settings::default())
            .unwrap(),
        vec![1, 2, 3]
    );
}
