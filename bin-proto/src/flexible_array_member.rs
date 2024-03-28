use crate::{BitRead, BitWrite, ByteOrder, Error};
use core::any::Any;

/// A trait for variable-length types without a length prefix.
pub trait FlexibleArrayMember: Sized {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error>;

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error>;
}
