use bin_proto::{Protocol, ByteOrder};

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
pub struct NamedFieldsWithGenerics<A: Protocol, D: Protocol> {
    pub value: A,
    pub del: D,
}

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct UnnamedFieldsWithGenerics<A: Protocol, D: Protocol>(A, D);

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct StructWithExistingBoundedGenerics<A: ::std::fmt::Display + ::std::fmt::Debug + Protocol>
{
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
        .bytes(ByteOrder::BigEndian)
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
        Foobar::from_bytes(&[3, '2' as u8, 1], ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_written() {
    assert_eq!(
        vec![6, 1, 9],
        BizBong(6, 1, 9).bytes(ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_read() {
    assert_eq!(
        BizBong(3, 1, 7),
        BizBong::from_bytes(&[3, 1, 7], ByteOrder::BigEndian).unwrap()
    );
}

#[test]
fn unit_structs_are_correctly_written() {
    assert_eq!(PartyInTheFront.bytes(ByteOrder::BigEndian).unwrap(), &[]);
}

#[test]
fn unit_structs_are_correctly_read() {
    assert_eq!(
        PartyInTheFront,
        PartyInTheFront::from_bytes(&[], ByteOrder::BigEndian).unwrap()
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
        IPv4Header::from_bytes(&[0x45], ByteOrder::BigEndian).unwrap(),
        IPv4Header { version: 4 }
    )
}
