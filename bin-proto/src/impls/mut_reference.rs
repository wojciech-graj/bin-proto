use crate::{BitWrite, ByteOrder, ProtocolWrite, Result};

impl<Ctx, T> ProtocolWrite<Ctx> for &mut T
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        (**self).write(write, byte_order, ctx)
    }
}

test_protocol_write!(&mut u8: &mut 1 => [0x01]);
