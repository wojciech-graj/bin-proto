/// A trait for types with discriminants (typically Enums).
pub trait Discriminable {
    type Discriminant;

    fn discriminant(&self) -> Self::Discriminant;
}
