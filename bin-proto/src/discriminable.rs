/// A trait for types with discriminants. Automatically derived for `enum`s.
pub trait Discriminable {
    type Discriminant;

    fn discriminant(&self) -> Self::Discriminant;
}
