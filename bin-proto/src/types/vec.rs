use crate::{util, BitRead, ByteOrder, FlexibleArrayMemberRead, ProtocolRead, Result};

impl<Ctx, T> FlexibleArrayMemberRead<Ctx> for Vec<T>
where
    T: ProtocolRead<Ctx>,
{
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        util::read_items_to_eof(read, byte_order, ctx)
    }
}
