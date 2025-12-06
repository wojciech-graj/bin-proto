/// A trait for types with discriminants. Automatically derived for `enum`s.
pub trait Discriminable {
    /// The type of the discriminant.
    type Discriminant;

    /// Returns a value uniquely identifying the variant.
    ///
    /// Returns [`None`] if the variant cannot be encoded.
    fn discriminant(&self) -> Option<Self::Discriminant>;
}
