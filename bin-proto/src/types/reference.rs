use crate::{BitWrite, ByteOrder, ProtocolWrite, Result};

impl<Ctx, T> ProtocolWrite<Ctx> for &T
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        (**self).write(write, byte_order, ctx)
    }
}

impl<Ctx, T> ProtocolWrite<Ctx> for &mut T
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        (**self).write(write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};
    use bitstream_io::{BigEndian, BitWriter};

    use super::*;

    #[test]
    fn can_write_reference() {
        let mut data: Vec<u8> = Vec::new();
        (&1u8)
            .write(
                &mut BitWriter::endian(&mut data, BigEndian),
                ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
        assert_eq!(data, vec![0x1]);
    }

    #[test]
    fn can_write_mut_reference() {
        let mut data: Vec<u8> = Vec::new();
        let mut value = 1u8;
        (&mut value)
            .write(
                &mut BitWriter::endian(&mut data, BigEndian),
                ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
        assert_eq!(data, vec![0x1]);
    }
}
