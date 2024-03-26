use crate::{BitRead, BitWrite, Error, Settings};
use core::any::Any;

/// A trait for variable-length types without a length prefix.
pub trait FlexibleArrayMember: Sized {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error>;

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error>;
}
