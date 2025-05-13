use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{util, BitDecode, BitEncode, Error, Result};
use alloc::vec::Vec;
use core::convert::TryInto;

impl<Ctx, T, const N: usize> BitDecode<Ctx> for [T; N]
where
    T: BitDecode<Ctx>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let elements: Vec<_> =
            util::decode_items::<_, E, _, _>(N, read, ctx).collect::<Result<_>>()?;
        elements.try_into().map_err(|_| Error::SliceTryFromVec)
    }
}

impl<Ctx, T, const N: usize> BitEncode<Ctx> for [T; N]
where
    T: BitEncode<Ctx> + Sized,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.iter(), write, ctx)
    }
}

test_codec!([u8; 4]; [0, 1, 2, 3] => [0x00, 0x01, 0x02, 0x03]);
