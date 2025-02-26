use core::ffi::CStr;

use crate::{util, BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};
use alloc::{ffi::CString, vec::Vec};

impl<Ctx> ProtocolRead<Ctx> for CString {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        let mut result = Vec::new();
        loop {
            let c: u8 = ProtocolRead::read(read, byte_order, ctx)?;
            if c == 0x00 {
                return Ok(Self::new(result)?);
            }
            result.push(c);
        }
    }
}

impl<Ctx> ProtocolWrite<Ctx> for CString {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.to_bytes_with_nul().iter(), write, byte_order, ctx)
    }
}

impl<Ctx> ProtocolWrite<Ctx> for CStr {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        util::write_items(self.to_bytes_with_nul().iter(), write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use super::*;

    #[test]
    fn can_read_cstring() {
        let mut data = BitReader::endian([0x41u8, 0x42, 0x43, 0].as_slice(), BigEndian);
        let read_back: CString =
            ProtocolRead::read(&mut data, ByteOrder::BigEndian, &mut ()).unwrap();
        assert_eq!(read_back, CString::new("ABC").unwrap());
    }

    #[test]
    fn can_write_cstring() {
        let mut data: Vec<u8> = Vec::new();
        CString::new("ABC")
            .unwrap()
            .write(
                &mut BitWriter::endian(&mut data, BigEndian),
                ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
        assert_eq!(data, vec![0x41, 0x42, 0x43, 0]);
    }

    #[test]
    fn can_write_cstr() {
        let exp = vec![0x41, 0x42, 0x43, 0];
        let mut data: Vec<u8> = Vec::new();
        CStr::from_bytes_until_nul(&exp)
            .unwrap()
            .write(
                &mut BitWriter::endian(&mut data, BigEndian),
                ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
        assert_eq!(data, exp);
    }
}
