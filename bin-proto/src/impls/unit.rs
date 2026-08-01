use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

impl<E, Ctx> BitDecode<E, Ctx> for ()
where
    E: Endianness,
{
    fn decode<R>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        Ok(())
    }
}

impl<E, Ctx> BitEncode<E, Ctx> for ()
where
    E: Endianness,
{
    fn encode<W>(&self, _: &mut W, _: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        Ok(())
    }
}

test_codec!((); () => []);
test_roundtrip!(());
