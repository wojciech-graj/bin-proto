//! Utilities for externally length prefixed fields

use crate::{BitRead, BitWrite, Error, Settings};
use core::any::Any;

/// A trait for variable-length types with a disjoint length prefix.
pub trait ExternallyLengthPrefixed: Sized {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        length: &FieldLength,
    ) -> Result<Self, Error>;

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error>;
}

pub type FieldIndex = usize;

/// Information about the length of a field.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldLength {
    pub length: usize,
    pub kind: LengthPrefixKind,
}

/// Specifies what kind of data the length prefix captures.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LengthPrefixKind {
    /// The length prefix stores the total number of bytes making up another field.
    Bytes,
    /// The length prefix stores the total number of elements inside another field.
    Elements,
}
