use crate::{util, BitRead, BitWrite, ByteOrder, Error, ProtocolRead, ProtocolWrite, Result};
use core::convert::TryInto;

impl<Ctx, T, const N: usize> ProtocolRead<Ctx> for [T; N]
where
    T: ProtocolRead<Ctx>,
{
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx, _: ()) -> Result<Self> {
        let elements = util::read_items(N, read, byte_order, ctx)?;
        elements.try_into().map_err(|_| Error::SliceTryFromVec)
    }
}

impl<Ctx, T, const N: usize> ProtocolWrite<Ctx> for [T; N]
where
    T: ProtocolWrite<Ctx> + Sized,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.iter(), write, byte_order, ctx)
    }
}

test_protocol!([u8; 4]: [0, 1, 2, 3] => [0x00, 0x01, 0x02, 0x03]);
