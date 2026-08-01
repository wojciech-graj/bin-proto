use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};
use core::marker::PhantomPinned;

impl<E, Ctx> BitDecode<E, Ctx> for PhantomPinned
where
    E: Endianness,
{
    fn decode<R>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        Ok(Self)
    }
}

impl<E, Ctx> BitEncode<E, Ctx> for PhantomPinned
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

test_codec!(PhantomPinned; PhantomPinned => []);
