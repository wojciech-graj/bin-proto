/// A trait for types with discriminants. Automatically derived for `enum`s.
pub trait Discriminable {
    /// The type of the discriminant
    type Discriminant;

    /// Returns a value uniquely identifying the variant
    fn discriminant(&self) -> Self::Discriminant;
}
