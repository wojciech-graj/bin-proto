use std::net::{Ipv4Addr, Ipv6Addr};

use crate::Protocol;

impl Protocol for Ipv4Addr {
    fn read(
        read: &mut dyn crate::BitRead,
        byte_order: crate::ByteOrder,
        ctx: &mut dyn std::any::Any,
    ) -> Result<Self, crate::Error> {
        let bytes: [u8; 4] = Protocol::read(read, byte_order, ctx)?;

        Ok(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }

    fn write(
        &self,
        write: &mut dyn crate::BitWrite,
        byte_order: crate::ByteOrder,
        ctx: &mut dyn std::any::Any,
    ) -> Result<(), crate::Error> {
        Protocol::write(&self.octets(), write, byte_order, ctx)
    }
}

impl Protocol for Ipv6Addr {
    fn read(
        read: &mut dyn crate::BitRead,
        byte_order: crate::ByteOrder,
        ctx: &mut dyn std::any::Any,
    ) -> Result<Self, crate::Error> {
        let bytes: [u16; 8] = Protocol::read(read, byte_order, ctx)?;

        Ok(Self::new(
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ))
    }

    fn write(
        &self,
        write: &mut dyn crate::BitWrite,
        byte_order: crate::ByteOrder,
        ctx: &mut dyn std::any::Any,
    ) -> Result<(), crate::Error> {
        Protocol::write(&self.octets(), write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use crate::ByteOrder;

    use super::*;

    #[test]
    fn read_ipv4_addr() {
        assert_eq!(
            <Ipv4Addr as Protocol>::read(
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
        Protocol::write(
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
            <Ipv6Addr as Protocol>::read(
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
        Protocol::write(
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
