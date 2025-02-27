use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Result};

impl<Ctx> BitDecode<Ctx> for () {
    fn decode(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<Self> {
        Ok(())
    }
}

impl<Ctx> BitEncode<Ctx> for () {
    fn encode(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx, (): ()) -> Result<()> {
        Ok(())
    }
}

test_codec!((); () => []);
