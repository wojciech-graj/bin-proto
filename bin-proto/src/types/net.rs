use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};

impl<Ctx> ProtocolRead<Ctx> for Ipv4Addr {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        let bytes: [u8; 4] = ProtocolRead::read(read, byte_order, ctx)?;

        Ok(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

impl<Ctx> ProtocolWrite<Ctx> for Ipv4Addr {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        ProtocolWrite::write(&self.octets(), write, byte_order, ctx)
    }
}

impl<Ctx> ProtocolRead<Ctx> for Ipv6Addr {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        let bytes: [u16; 8] = ProtocolRead::read(read, byte_order, ctx)?;

        Ok(Self::new(
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ))
    }
}

impl<Ctx> ProtocolWrite<Ctx> for Ipv6Addr {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        ProtocolWrite::write(&self.octets(), write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use super::*;

    #[test]
    fn read_ipv4_addr() {
        assert_eq!(
            <Ipv4Addr as ProtocolRead>::read(
                &mut BitReader::endian([192u8, 168, 1, 0].as_slice(), BigEndian),
                ByteOrder::BigEndian,
                &mut ()
            )
            .unwrap(),
            Ipv4Addr::new(192, 168, 1, 0)
        )
    }

    #[test]
    fn write_ipv4_addr() {
        let mut data: Vec<u8> = Vec::new();
        ProtocolWrite::write(
            &Ipv4Addr::new(192, 168, 1, 0),
            &mut BitWriter::endian(&mut data, BigEndian),
            ByteOrder::BigEndian,
            &mut (),
        )
        .unwrap();
        assert_eq!(vec![192, 168, 1, 0], data);
    }

    #[test]
    fn read_ipv6_addr() {
        assert_eq!(
            <Ipv6Addr as ProtocolRead>::read(
                &mut BitReader::endian(
                    [
                        0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e,
                        0x03, 0x70, 0x73, 0x34
                    ]
                    .as_slice(),
                    BigEndian
                ),
                ByteOrder::BigEndian,
                &mut ()
            )
            .unwrap(),
            Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334)
        )
    }

    #[test]
    fn write_ipv6_addr() {
        let mut data: Vec<u8> = Vec::new();
        ProtocolWrite::write(
            &Ipv6Addr::new(
                0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334,
            ),
            &mut BitWriter::endian(&mut data, BigEndian),
            ByteOrder::BigEndian,
            &mut (),
        )
        .unwrap();
        assert_eq!(
            vec![
                0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e, 0x03, 0x70,
                0x73, 0x34
            ],
            data
        );
    }
}
