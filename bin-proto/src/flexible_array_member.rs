use crate::{BitRead, ByteOrder, Result};

/// A trait for variable-length types without a length prefix.
pub trait FlexibleArrayMemberRead<Ctx = ()>: Sized {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self>;
}
