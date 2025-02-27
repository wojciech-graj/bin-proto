use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Result};
use core::marker::PhantomData;

impl<Ctx, T> BitDecode<Ctx> for PhantomData<T> {
    fn decode(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<Self> {
        Ok(Self)
    }
}

impl<Ctx, T> BitEncode<Ctx> for PhantomData<T> {
    fn encode(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<()> {
        Ok(())
    }
}

test_codec!(PhantomData<u8>; PhantomData => []);
