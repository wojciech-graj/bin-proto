use bitstream_io::{BigEndian, BitReader, BitWriter, LittleEndian};

use crate::{BitRead, BitWrite, ByteOrder, Error};
use core::any::Any;
use std::io;

/// A trait for bit-level co/dec.
pub trait Protocol: Sized {
    /// Reads self from a stream.
    ///
    /// Blocks until a value is received.
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error>;

    /// Writes a value to a stream.
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error>;

    /// Parses a new value from its raw byte representation without context.
    fn from_bytes(bytes: &[u8], byte_order: ByteOrder) -> Result<Self, Error> {
        Self::from_bytes_ctx(bytes, byte_order, &mut ())
    }

    /// Parses a new value from its raw byte representation with additional context.
    fn from_bytes_ctx(
        bytes: &[u8],
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        match byte_order {
            crate::ByteOrder::LittleEndian => {
                let mut buffer = BitReader::endian(io::Cursor::new(bytes), LittleEndian);
                Self::read(&mut buffer, byte_order, ctx)
            }
            crate::ByteOrder::BigEndian => {
                let mut buffer = BitReader::endian(io::Cursor::new(bytes), BigEndian);
                Self::read(&mut buffer, byte_order, ctx)
            }
        }
    }

    /// Gets the raw bytes of this type without context.
    fn bytes(&self, byte_order: ByteOrder) -> Result<Vec<u8>, Error> {
        self.bytes_ctx(byte_order, &mut ())
    }

    /// Gets the raw bytes of this type with provided context.
    fn bytes_ctx(&self, byte_order: ByteOrder, ctx: &mut dyn Any) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        match byte_order {
            crate::ByteOrder::LittleEndian => {
                let mut writer = BitWriter::endian(&mut data, LittleEndian);
                self.write(&mut writer, byte_order, ctx)?;
                writer.byte_align()?;
            }
            crate::ByteOrder::BigEndian => {
                let mut writer = BitWriter::endian(&mut data, BigEndian);
                self.write(&mut writer, byte_order, ctx)?;
                writer.byte_align()?;
            }
        };

        Ok(data)
    }
}
