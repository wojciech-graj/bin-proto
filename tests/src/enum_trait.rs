use protocol::Enum;

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub enum WithGenerics<A, B> {
    Foo(A, B),
    Bar,
}

#[test]
fn can_get_discriminant() {
    let foo = WithGenerics::Foo(99u16, "hello".to_owned());
    let bar: WithGenerics<bool, bool> = WithGenerics::Bar;

    assert_eq!("Foo", foo.discriminant());
    assert_eq!("Bar", bar.discriminant());
}
