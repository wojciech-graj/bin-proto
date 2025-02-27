use core::net::Ipv4Addr;

use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Result};

impl<Ctx> BitDecode<Ctx> for Ipv4Addr {
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: (),
    ) -> Result<Self> {
        let bytes: [u8; 4] = BitDecode::decode(read, byte_order, ctx, tag)?;

        Ok(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

impl<Ctx> BitEncode<Ctx> for Ipv4Addr {
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        self.octets().encode(write, byte_order, ctx, ())
    }
}

test_codec!(Ipv4Addr; Ipv4Addr::new(192, 168, 1, 0) => [192, 168, 1, 0]);
