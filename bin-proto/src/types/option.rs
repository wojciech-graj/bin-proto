use crate::{BitField, BitRead, BitWrite, Error, Protocol, Settings};
use core::any::Any;

impl<T: Protocol> Protocol for Option<T> {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        let is_some = <bool as Protocol>::read(read, settings, ctx)?;

        if is_some {
            let value = T::read(read, settings, ctx)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        Protocol::write(&self.is_some(), write, settings, ctx)?;

        if let Some(ref value) = *self {
            value.write(write, settings, ctx)?;
        }
        Ok(())
    }
}

impl<T: Protocol> BitField for Option<T> {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        bits: u32,
    ) -> Result<Self, Error> {
        let is_some = <bool as BitField>::read(read, settings, ctx, bits)?;

        if is_some {
            let value = T::read(read, settings, ctx)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
        bits: u32,
    ) -> Result<(), Error> {
        BitField::write(&self.is_some(), write, settings, ctx, bits)?;

        if let Some(ref value) = *self {
            value.write(write, settings, ctx)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use super::*;

    #[test]
    fn can_read_some() {
        assert_eq!(
            Option::<u8>::from_bytes(&[1, 5], &Settings::default()).unwrap(),
            Some(5)
        )
    }

    #[test]
    fn can_read_none() {
        assert_eq!(
            Option::<u8>::from_bytes(&[0], &Settings::default()).unwrap(),
            None
        )
    }

    #[test]
    fn can_write_some() {
        assert_eq!(Some(5u8).bytes(&Settings::default()).unwrap(), &[1, 5])
    }

    #[test]
    fn can_write_none() {
        assert_eq!(None::<u8>.bytes(&Settings::default()).unwrap(), &[0])
    }

    #[test]
    fn can_read_some_bitfield() {
        assert_eq!(
            <Option::<u8> as BitField>::read(
                &mut BitReader::endian(Cursor::new([0x82u8, 0x80]), BigEndian),
                &Settings::default(),
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
                &Settings::default(),
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
        BitField::write(&Some(5u8), &mut writer, &Settings::default(), &mut (), 1).unwrap();
        writer.byte_align().unwrap();
        assert_eq!(vec![0x82, 0x80], buffer)
    }

    #[test]
    fn can_write_none_bitfield() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = BitWriter::endian(&mut buffer, BigEndian);
        BitField::write(&None::<u8>, &mut writer, &Settings::default(), &mut (), 1).unwrap();
        writer.byte_align().unwrap();
        assert_eq!(vec![0x00], buffer)
    }
}
