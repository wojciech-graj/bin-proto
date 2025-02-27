use core::ffi::CStr;

use crate::{util, BitWrite, ByteOrder, ProtocolWrite, Result};

impl<Ctx> ProtocolWrite<Ctx> for CStr {
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        util::write_items(self.to_bytes_with_nul().iter(), write, byte_order, ctx)
    }
}

test_protocol_write!(&CStr; CStr::from_bytes_with_nul(&[0x41, 0x42, 0x43, 0]).unwrap() => [0x41, 0x42, 0x43, 0]);
