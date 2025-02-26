use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};
use core::marker::PhantomData;

impl<Ctx, T> ProtocolRead<Ctx> for PhantomData<T> {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        Ok(Self)
    }
}

impl<Ctx, T> ProtocolWrite<Ctx> for PhantomData<T> {
    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

test_protocol!(PhantomData<u8>: PhantomData => []);
