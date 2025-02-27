use alloc::{ffi::CString, vec::Vec};

use crate::{util, BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Result};

impl<Ctx> BitDecode<Ctx> for CString {
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: (),
    ) -> Result<Self> {
        let mut result = Vec::new();
        loop {
            let c: u8 = BitDecode::decode(read, byte_order, ctx, tag)?;
            if c == 0x00 {
                return Ok(Self::new(result)?);
            }
            result.push(c);
        }
    }
}

impl<Ctx> BitEncode<Ctx> for CString {
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

test_codec!(CString; CString::new("ABC").unwrap() => [0x41, 0x42, 0x43, 0]);
