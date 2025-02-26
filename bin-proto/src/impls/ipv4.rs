use core::net::Ipv4Addr;

use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};

impl<Ctx> ProtocolRead<Ctx> for Ipv4Addr {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        let bytes: [u8; 4] = ProtocolRead::read(read, byte_order, ctx)?;

        Ok(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

impl<Ctx> ProtocolWrite<Ctx> for Ipv4Addr {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        self.octets().write(write, byte_order, ctx)
    }
}

test_protocol!(Ipv4Addr: Ipv4Addr::new(192, 168, 1, 0) => [192, 168, 1, 0]);
