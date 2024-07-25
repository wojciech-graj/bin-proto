use crate::{BitField, BitRead, BitWrite, ByteOrder, Protocol, Result};

impl<Ctx, T> Protocol<Ctx> for Option<T>
where
    T: Protocol<Ctx>,
{
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        let is_some = <bool as Protocol<Ctx>>::read(read, byte_order, ctx)?;

        if is_some {
            let value = T::read(read, byte_order, ctx)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        Protocol::write(&self.is_some(), write, byte_order, ctx)?;

        if let Some(ref value) = *self {
            value.write(write, byte_order, ctx)?;
        }
        Ok(())
    }
}

impl<Ctx, T> BitField<Ctx> for Option<T>
where
    T: Protocol<Ctx>,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        bits: u32,
    ) -> Result<Self> {
        let is_some = <bool as BitField<Ctx>>::read(read, byte_order, ctx, bits)?;

        if is_some {
            let value = T::read(read, byte_order, ctx)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        bits: u32,
    ) -> Result<()> {
        BitField::write(&self.is_some(), write, byte_order, ctx, bits)?;

        if let Some(ref value) = *self {
            value.write(write, byte_order, ctx)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use crate::ProtocolNoCtx;

    use super::*;

    #[test]
    fn can_read_some() {
        assert_eq!(
            Option::<u8>::from_bytes(&[1, 5], ByteOrder::BigEndian).unwrap(),
            Some(5)
        )
    }

    #[test]
    fn can_read_none() {
        assert_eq!(
            Option::<u8>::from_bytes(&[0], ByteOrder::BigEndian).unwrap(),
            None
        )
    }

    #[test]
    fn can_write_some() {
        assert_eq!(Some(5u8).bytes(ByteOrder::BigEndian).unwrap(), &[1, 5])
    }

    #[test]
    fn can_write_none() {
        assert_eq!(None::<u8>.bytes(ByteOrder::BigEndian).unwrap(), &[0])
    }

    #[test]
    fn can_read_some_bitfield() {
        assert_eq!(
            <Option::<u8> as BitField>::read(
                &mut BitReader::endian(Cursor::new([0x82u8, 0x80]), BigEndian),
                ByteOrder::BigEndian,
                &mut (),
                1,
            )
            .unwrap(),
            Some(5)
        )
    }

    #[test]
    fn can_read_none_bitfield() {
        assert_eq!(
            <Option::<u8> as BitField>::read(
                &mut BitReader::endian(Cursor::new([0x00]), BigEndian),
                ByteOrder::BigEndian,
                &mut (),
                1,
            )
            .unwrap(),
            None
        )
    }

    #[test]
    fn can_write_some_bitfield() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = BitWriter::endian(&mut buffer, BigEndian);
        BitField::write(&Some(5u8), &mut writer, ByteOrder::BigEndian, &mut (), 1).unwrap();
        writer.byte_align().unwrap();
        assert_eq!(vec![0x82, 0x80], buffer)
    }

    #[test]
    fn can_write_none_bitfield() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = BitWriter::endian(&mut buffer, BigEndian);
        BitField::write(&None::<u8>, &mut writer, ByteOrder::BigEndian, &mut (), 1).unwrap();
        writer.byte_align().unwrap();
        assert_eq!(vec![0x00], buffer)
    }
}
