use crate::{util, BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Error, Result};
use core::convert::TryInto;

impl<Ctx, T, const N: usize> BitDecode<Ctx> for [T; N]
where
    T: BitDecode<Ctx>,
{
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<Self> {
        let elements = util::decode_items(N, read, byte_order, ctx)?;
        elements.try_into().map_err(|_| Error::SliceTryFromVec)
    }
}

impl<Ctx, T, const N: usize> BitEncode<Ctx> for [T; N]
where
    T: BitEncode<Ctx> + Sized,
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

test_codec!([u8; 4]; [0, 1, 2, 3] => [0x00, 0x01, 0x02, 0x03]);
