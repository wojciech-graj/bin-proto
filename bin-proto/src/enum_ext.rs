use crate::Protocol;

/// A trait for `enum`s with the `Protocol` trait.
pub trait EnumExt: Protocol {
    /// The type used to store the enum discriminant
    type Discriminant: Protocol;

    /// Gets the discriminant of the current variant.
    fn discriminant(&self) -> Self::Discriminant;
}
