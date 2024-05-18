use std::marker::PhantomData;

use bin_proto::{ByteOrder, Protocol};

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct Foobar {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct BizBong(u8, u8, pub u8);

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct PartyInTheFront;

#[derive(Protocol, Debug, PartialEq, Eq)]
#[protocol(ctx = "()")]
pub struct NamedFieldsWithGenerics<A: Protocol, D: Protocol> {
    pub value: A,
    pub del: D,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
#[protocol(ctx = "Ctx")]
pub struct UnnamedFieldsWithGenerics<Ctx, A: Protocol<Ctx>, D: Protocol<Ctx>>(
    A,
    D,
    PhantomData<Ctx>,
);

#[derive(Protocol, Debug, PartialEq, Eq)]
#[protocol(ctx = "()")]
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
        .bytes_ctx(ByteOrder::BigEndian, &mut ())
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
        Foobar::from_bytes_ctx(&[3, '2' as u8, 1], ByteOrder::BigEndian, &mut ()).unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_written() {
    assert_eq!(
        vec![6, 1, 9],
        BizBong(6, 1, 9)
            .bytes_ctx(ByteOrder::BigEndian, &mut ())
            .unwrap()
    );
}

#[test]
fn unnamed_fields_are_correctly_read() {
    assert_eq!(
        BizBong(3, 1, 7),
        BizBong::from_bytes_ctx(&[3, 1, 7], ByteOrder::BigEndian, &mut ()).unwrap()
    );
}

#[test]
fn unit_structs_are_correctly_written() {
    assert_eq!(
        PartyInTheFront
            .bytes_ctx(ByteOrder::BigEndian, &mut ())
            .unwrap(),
        &[]
    );
}

#[test]
fn unit_structs_are_correctly_read() {
    assert_eq!(
        PartyInTheFront,
        PartyInTheFront::from_bytes_ctx(&[], ByteOrder::BigEndian, &mut ()).unwrap()
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
        IPv4Header::from_bytes_ctx(&[0x45], ByteOrder::BigEndian, &mut ()).unwrap(),
        IPv4Header { version: 4 }
    )
}
