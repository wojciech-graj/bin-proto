use bin_proto::{Protocol, Settings};

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct Foobar {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct BizBong(u8, u8, pub u8);

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct PartyInTheFront;

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct NamedFieldsWithGenerics<A, D> {
    pub value: A,
    pub del: D,
}

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct UnnamedFieldsWithGenerics<A, D>(A, D);

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct StructWithExistingBoundedGenerics<A: ::std::fmt::Display + ::std::fmt::Debug> {
    foo: A,
}

#[test]
fn named_fields_are_correctly_written() {
    assert_eq!(
        vec![3, '2' as u8, 1],
        Foobar {
            a: 3,
            b: '2' as u8,
            c: 1,
        }
        .bytes(&Settings::default())
        .unwrap()
    );
}

#[test]
fn named_fields_are_correctly_read() {
    assert_eq!(
        Foobar {
            a: 3,
            b: '2' as u8,
            c: 1,
        },
        Foobar::from_bytes(&[3, '2' as u8, 1], &Settings::default()).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_written() {
    assert_eq!(
        vec![6, 1, 9],
        BizBong(6, 1, 9).bytes(&Settings::default()).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_read() {
    assert_eq!(
        BizBong(3, 1, 7),
        BizBong::from_bytes(&[3, 1, 7], &Settings::default()).unwrap()
    );
}

#[test]
fn unit_structs_are_correctly_written() {
    assert_eq!(
        PartyInTheFront.bytes(&Settings::default()).unwrap(),
        &[]
    );
}

#[test]
fn unit_structs_are_correctly_read() {
    assert_eq!(
        PartyInTheFront,
        PartyInTheFront::from_bytes(&[], &Settings::default()).unwrap()
    );
}

#[test]
fn ipv4() {
    #[derive(Debug, bin_proto::Protocol, PartialEq, Eq)]
    struct IPv4Header {
        #[protocol(bits = 4)]
        version: u8,
    }

    assert_eq!(
        IPv4Header::from_bytes(&[0x45], &Settings::default()).unwrap(),
        IPv4Header { version: 4 }
    )
}
