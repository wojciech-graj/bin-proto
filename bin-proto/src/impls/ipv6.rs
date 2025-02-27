use core::net::Ipv6Addr;

use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};

impl<Ctx> ProtocolRead<Ctx> for Ipv6Addr {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx, tag: ()) -> Result<Self> {
        let bytes: [u16; 8] = ProtocolRead::read(read, byte_order, ctx, tag)?;

        Ok(Self::new(
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ))
    }
}

impl<Ctx> ProtocolWrite<Ctx> for Ipv6Addr {
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<()> {
        self.octets().write(write, byte_order, ctx, ())
    }
}

test_protocol!(Ipv6Addr;
    Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334) =>
    [
        0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e, 0x03, 0x70, 0x73,
        0x34
    ]
);
