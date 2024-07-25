use crate::{util, BitRead, BitWrite, ByteOrder, FlexibleArrayMember, Protocol, Result};

impl<Ctx, T> FlexibleArrayMember<Ctx> for Vec<T>
where
    T: Protocol<Ctx>,
{
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        util::read_items_to_eof(read, byte_order, ctx)
    }

    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.iter(), write, byte_order, ctx)
    }
}
