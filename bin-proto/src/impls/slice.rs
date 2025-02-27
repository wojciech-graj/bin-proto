use crate::{util, BitEncode, BitWrite, ByteOrder, Result};

impl<Ctx, T> BitEncode<Ctx> for [T]
where
    T: BitEncode<Ctx>,
{
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        util::encode_items(self.iter(), write, byte_order, ctx)
    }
}

test_encode!(&[u8]; &[1, 2, 3] => [0x01, 0x02, 0x03]);
