use crate::{util, BitWrite, ByteOrder, Result, UntaggedWrite};

use alloc::vec::Vec;

impl<Ctx> UntaggedWrite<Ctx> for str {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        let bytes: Vec<_> = self.bytes().collect();
        util::write_items(&bytes, write, byte_order, ctx)
    }
}

test_untagged_write!(&str: "abc" => [b'a', b'b', b'c']);
