use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};
use core::marker::PhantomPinned;

impl<Ctx> ProtocolRead<Ctx> for PhantomPinned {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<Self> {
        Ok(Self)
    }
}

impl<Ctx> ProtocolWrite<Ctx> for PhantomPinned {
    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<()> {
        Ok(())
    }
}

test_protocol!(PhantomPinned; PhantomPinned => []);
