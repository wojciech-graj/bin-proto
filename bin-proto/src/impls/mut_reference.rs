use crate::{BitEncode, BitWrite, ByteOrder, Result};

impl<Ctx, Tag, T> BitEncode<Ctx, Tag> for &mut T
where
    T: BitEncode<Ctx, Tag>,
{
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<()> {
        (**self).encode(write, byte_order, ctx, tag)
    }
}

test_encode!(&mut u8; &mut 1 => [0x01]);
