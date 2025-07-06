use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

impl<Ctx> BitDecode<Ctx> for () {
    fn decode<R, E>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        Ok(())
    }
}

impl<Ctx> BitEncode<Ctx> for () {
    fn encode<W, E>(&self, _: &mut W, _: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        Ok(())
    }
}

test_codec!((); () => []);
test_roundtrip!(());
