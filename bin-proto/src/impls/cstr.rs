use core::ffi::CStr;

use crate::{util, BitEncode, BitWrite, ByteOrder, Result};

impl<Ctx> BitEncode<Ctx> for CStr {
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        util::encode_items(self.to_bytes_with_nul().iter(), write, byte_order, ctx)
    }
}

test_encode!(
    &CStr; CStr::from_bytes_with_nul(&[0x41, 0x42, 0x43, 0]).unwrap() => [0x41, 0x42, 0x43, 0]
);
