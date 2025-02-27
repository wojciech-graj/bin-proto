use crate::{util, BitEncode, BitWrite, ByteOrder, Result};

use alloc::vec::Vec;

impl<Ctx> BitEncode<Ctx> for str {
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        let bytes: Vec<_> = self.bytes().collect();
        util::encode_items(&bytes, write, byte_order, ctx)
    }
}

test_encode!(&str; "abc" => [b'a', b'b', b'c']);
