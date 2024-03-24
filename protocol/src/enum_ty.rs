use crate::Parcel;

/// An `enum` type.
pub trait Enum: Parcel {
    /// The type used to store the enum discriminant
    type Discriminant: Parcel;

    /// Gets the discriminant of the current variant.
    fn discriminant(&self) -> Self::Discriminant;
}
