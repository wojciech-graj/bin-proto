use crate::{BitRead, BitWrite, ByteOrder, Error, Protocol};
use std::convert::TryInto;

impl<Ctx, T: Protocol<Ctx> + std::fmt::Debug, const N: usize> Protocol<Ctx> for [T; N] {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self, Error> {
        let elements = crate::util::read_items(N, read, byte_order, ctx)?;
        Ok(elements.into_iter().collect::<Vec<T>>().try_into().unwrap())
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
    ) -> Result<(), Error> {
        crate::util::write_items(self.iter(), write, byte_order, ctx)
    }
}

#[cfg(test)]
mod test {
    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use crate::{ByteOrder, Protocol};
    use std::io::Cursor;

    #[test]
    fn can_read_array() {
        let mut data = BitReader::endian(Cursor::new([0u8, 1, 2, 3]), BigEndian);
        let read_back: [u8; 4] = Protocol::read(&mut data, ByteOrder::BigEndian, &mut ()).unwrap();
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
}
