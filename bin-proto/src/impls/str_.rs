use crate::{util, BitWrite, ByteOrder, ProtocolWrite, Result};

use alloc::vec::Vec;

impl<Ctx> ProtocolWrite<Ctx> for str {
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        let bytes: Vec<_> = self.bytes().collect();
        util::write_items(&bytes, write, byte_order, ctx)
    }
}

test_protocol_write!(&str; "abc" => [b'a', b'b', b'c']);
