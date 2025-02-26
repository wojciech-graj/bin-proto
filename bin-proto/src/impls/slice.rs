use crate::{util, BitWrite, ByteOrder, ProtocolWrite, Result, UntaggedWrite};

impl<Ctx, T> UntaggedWrite<Ctx> for [T]
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.iter(), write, byte_order, ctx)
    }
}

test_untagged_write!(&[u8]: &[1, 2, 3] => [0x01, 0x02, 0x03]);
