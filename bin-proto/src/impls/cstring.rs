use alloc::{ffi::CString, vec::Vec};

use crate::{util, BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};

impl<Ctx> ProtocolRead<Ctx> for CString {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx, tag: ()) -> Result<Self> {
        let mut result = Vec::new();
        loop {
            let c: u8 = ProtocolRead::read(read, byte_order, ctx, tag)?;
            if c == 0x00 {
                return Ok(Self::new(result)?);
            }
            result.push(c);
        }
    }
}

impl<Ctx> ProtocolWrite<Ctx> for CString {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.to_bytes_with_nul().iter(), write, byte_order, ctx)
    }
}

test_protocol!(CString: CString::new("ABC").unwrap() => [0x41, 0x42, 0x43, 0]);
