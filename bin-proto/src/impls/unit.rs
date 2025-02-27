use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};

impl<Ctx> ProtocolRead<Ctx> for () {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<Self> {
        Ok(())
    }
}

impl<Ctx> ProtocolWrite<Ctx> for () {
    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

test_protocol!((): () => []);
