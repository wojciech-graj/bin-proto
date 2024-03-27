use bin_proto::{Enum, Protocol};

#[derive(bin_proto::Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant_type = "u8")]
pub enum WithGenerics<A: Protocol, B: Protocol> {
    #[protocol(discriminant = "1")]
    Foo(A, B),
    #[protocol(discriminant = "2")]
    Bar,
}

#[test]
fn can_get_discriminant() {
    let foo = WithGenerics::Foo(99u16, false);
    let bar: WithGenerics<bool, bool> = WithGenerics::Bar;

    assert_eq!(1, foo.discriminant());
    assert_eq!(2, bar.discriminant());
}
