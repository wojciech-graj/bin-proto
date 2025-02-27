use crate::{BitWrite, ByteOrder, ProtocolWrite, Result};

impl<Ctx, Tag, T> ProtocolWrite<Ctx, Tag> for &T
where
    T: ProtocolWrite<Ctx, Tag>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        (**self).write(write, byte_order, ctx)
    }
}

test_protocol_write!(&u8: &1 => [0x01]);
