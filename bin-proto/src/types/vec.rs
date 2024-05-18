use crate::{util, BitRead, BitWrite, ByteOrder, Error, FlexibleArrayMember, Protocol};

impl<Ctx, T: Protocol<Ctx>> FlexibleArrayMember<Ctx> for Vec<T> {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self, Error> {
        util::read_items_to_eof(read, byte_order, ctx)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
    ) -> Result<(), Error> {
        util::write_items(self.iter(), write, byte_order, ctx)
    }
}
