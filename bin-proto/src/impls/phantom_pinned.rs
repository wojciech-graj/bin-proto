use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Result};
use core::marker::PhantomPinned;

impl<Ctx> BitDecode<Ctx> for PhantomPinned {
    fn decode(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<Self> {
        Ok(Self)
    }
}

impl<Ctx> BitEncode<Ctx> for PhantomPinned {
    fn encode(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<()> {
        Ok(())
    }
}

test_codec!(PhantomPinned; PhantomPinned => []);
