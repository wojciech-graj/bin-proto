use crate::{util, BitRead, BitWrite, Error, Protocol, Settings};
use std::ffi::CString;

impl Protocol for CString {
    fn read(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let mut result = Vec::new();
        // this logic is susceptible to DoS attacks by never providing
        //   a null character and will be fixed by
        //   https://github.com/dylanmckay/bin_proto/issues/14
        loop {
            let c: u8 = Protocol::read(read, settings)?;
            if c == 0x00 {
                return Ok(CString::new(result)?);
            }
            result.push(c);
        }
    }

    fn write(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        util::write_items(self.clone().into_bytes_with_nul().iter(), write, settings)
    }
}

#[cfg(test)]
mod test {
    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use crate::{Protocol, Settings};
    use std::ffi::CString;
    use std::io::Cursor;

    #[test]
    fn can_read_cstring() {
        let mut data = BitReader::endian(Cursor::new([0x41, 0x42, 0x43, 0]), BigEndian);
        let read_back: CString = Protocol::read(&mut data, &Settings::default()).unwrap();
        assert_eq!(read_back, CString::new("ABC").unwrap());
    }

    #[test]
    fn can_write_cstring() {
        let mut data = Cursor::new(Vec::new());
        let mut buffer = BitWriter::endian(&mut data, BigEndian);

        CString::new("ABC")
            .unwrap()
            .write(&mut buffer, &Settings::default())
            .unwrap();
        assert_eq!(data.into_inner(), vec![0x41, 0x42, 0x43, 0]);
    }
}
