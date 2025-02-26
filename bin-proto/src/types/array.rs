use crate::{
    util, BitRead, BitWrite, ByteOrder, Error, ProtocolRead, ProtocolWrite, Result, UntaggedWrite,
};
use core::convert::TryInto;

impl<Ctx, T, const N: usize> ProtocolRead<Ctx> for [T; N]
where
    T: ProtocolRead<Ctx>,
{
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        let elements = util::read_items(N, read, byte_order, ctx)?;
        elements.try_into().map_err(|_| Error::SliceTryFromVec)
    }
}

impl<Ctx, T, const N: usize> ProtocolWrite<Ctx> for [T; N]
where
    T: ProtocolWrite<Ctx> + Sized,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.iter(), write, byte_order, ctx)
    }
}

impl<Ctx, T> UntaggedWrite<Ctx> for [T]
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.iter(), write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};
    #[cfg(feature = "std")]
    use std::io::Cursor;

    use bitstream_io::{BigEndian, BitReader, BitWriter};
    #[cfg(not(feature = "std"))]
    use core2::io::Cursor;

    use super::*;

    #[test]
    fn can_read_array() {
        let mut data = BitReader::endian(Cursor::new([0u8, 1, 2, 3]), BigEndian);
        let read_back: [u8; 4] =
            ProtocolRead::read(&mut data, ByteOrder::BigEndian, &mut ()).unwrap();
        assert_eq!(read_back, [0, 1, 2, 3]);
    }

    #[test]
    fn can_write_array() {
        let mut data = Vec::new();
        let mut writer = BitWriter::endian(&mut data, BigEndian);

        [5u8, 7, 9, 11]
            .write(&mut writer, ByteOrder::BigEndian, &mut ())
            .unwrap();
        assert_eq!(data, vec![5, 7, 9, 11]);
    }

    #[test]
    fn can_write_slice() {
        let mut data = Vec::new();
        let mut writer = BitWriter::endian(&mut data, BigEndian);

        [5u8, 7, 9, 11]
            .as_slice()
            .write(&mut writer, ByteOrder::BigEndian, &mut ())
            .unwrap();
        assert_eq!(data, vec![5, 7, 9, 11]);
    }
}
