use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};
use core::marker::PhantomPinned;

impl<Ctx> BitDecode<Ctx> for PhantomPinned {
    fn decode<R, E>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        Ok(Self)
    }
}

impl<Ctx> BitEncode<Ctx> for PhantomPinned {
    fn encode<W, E>(&self, _: &mut W, _: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        Ok(())
    }
}

test_codec!(PhantomPinned; PhantomPinned => []);
