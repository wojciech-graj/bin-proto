use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};
use core::marker::PhantomData;

impl<E, Ctx, T> BitDecode<E, Ctx> for PhantomData<T>
where
    E: Endianness,
    T: ?Sized,
{
    fn decode<R>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        Ok(Self)
    }
}

impl<E, Ctx, T> BitEncode<E, Ctx> for PhantomData<T>
where
    E: Endianness,
    T: ?Sized,
{
    fn encode<W>(&self, _: &mut W, _: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        Ok(())
    }
}

test_codec!(PhantomData<u8>; PhantomData => []);
test_roundtrip!(PhantomData<u8>);
