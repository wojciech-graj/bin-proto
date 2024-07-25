use crate::{BitRead, BitWrite, ByteOrder, Result};

/// A trait for variable-length types without a length prefix.
pub trait FlexibleArrayMember<Ctx = ()>: Sized {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self>;

    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()>;
}
