use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use core::any::Any;
use std::convert::TryInto;

impl<T: Protocol + std::fmt::Debug, const N: usize> Protocol for [T; N] {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        let elements = crate::util::read_items(N, read, settings, ctx)?;
        Ok(elements.into_iter().collect::<Vec<T>>().try_into().unwrap())
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        crate::util::write_list(self.iter(), write, settings, ctx)
    }
}

#[cfg(test)]
mod test {
    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use crate::{Protocol, Settings};
    use std::io::Cursor;

    #[test]
    fn can_read_array() {
        let mut data = BitReader::endian(Cursor::new([0u8, 1, 2, 3]), BigEndian);
        let read_back: [u8; 4] = Protocol::read(&mut data, &Settings::default(), &mut ()).unwrap();
        assert_eq!(read_back, [0, 1, 2, 3]);
    }

    #[test]
    fn can_write_array() {
        let mut data = Vec::new();
        let mut writer = BitWriter::endian(&mut data, BigEndian);

        [5u8, 7, 9, 11]
            .write(&mut writer, &Settings::default(), &mut ())
            .unwrap();
        assert_eq!(data, vec![5, 7, 9, 11]);
    }
}
