use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};
use core::marker::PhantomData;

impl<Ctx, T> BitDecode<Ctx> for PhantomData<T> {
    fn decode<R, E>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        Ok(Self)
    }
}

impl<Ctx, T> BitEncode<Ctx> for PhantomData<T> {
    fn encode<W, E>(&self, _: &mut W, _: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        Ok(())
    }
}

test_codec!(PhantomData<u8>; PhantomData => []);
test_roundtrip!(PhantomData<u8>);
