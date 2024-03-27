//! Utilities for externally length prefixed fields

use crate::{BitRead, BitWrite, Error, Settings};
use core::any::Any;

/// A trait for variable-length types with a disjoint length prefix.
pub trait ExternallyLengthPrefixed: Sized {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        length: usize,
    ) -> Result<Self, Error>;

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error>;
}
