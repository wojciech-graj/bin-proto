use bitstream_io::{BigEndian, BitReader, BitWriter, LittleEndian};

use crate::{BitRead, BitWrite, ByteOrder, Result};
use std::io;

/// A trait for bit-level decoding.
pub trait ProtocolRead<Ctx = ()>: Sized {
    /// Reads self from a stream.
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self>;

    /// Parses a new value from its raw byte representation with additional context.
    fn from_bytes_ctx(bytes: &[u8], byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        match byte_order {
            ByteOrder::LittleEndian => {
                let mut buffer = BitReader::endian(io::Cursor::new(bytes), LittleEndian);
                Self::read(&mut buffer, byte_order, ctx)
            }
            ByteOrder::BigEndian => {
                let mut buffer = BitReader::endian(io::Cursor::new(bytes), BigEndian);
                Self::read(&mut buffer, byte_order, ctx)
            }
        }
    }
}

/// A trait for bit-level encoding.
pub trait ProtocolWrite<Ctx = ()> {
    /// Writes a value to a stream.
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()>;

    /// Gets the raw bytes of this type with provided context.
    fn bytes_ctx(&self, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        match byte_order {
            ByteOrder::LittleEndian => {
                let mut writer = BitWriter::endian(&mut data, LittleEndian);
                self.write(&mut writer, byte_order, ctx)?;
                writer.byte_align()?;
            }
            ByteOrder::BigEndian => {
                let mut writer = BitWriter::endian(&mut data, BigEndian);
                self.write(&mut writer, byte_order, ctx)?;
                writer.byte_align()?;
            }
        };

        Ok(data)
    }
}

/// A trait with helper functions for contextless `Protocol`s
pub trait ProtocolNoCtx: ProtocolRead + ProtocolWrite {
    /// Parses a new value from its raw byte representation without context.
    fn from_bytes(bytes: &[u8], byte_order: ByteOrder) -> Result<Self> {
        Self::from_bytes_ctx(bytes, byte_order, &mut ())
    }

    /// Gets the raw bytes of this type without context.
    fn bytes(&self, byte_order: ByteOrder) -> Result<Vec<u8>> {
        self.bytes_ctx(byte_order, &mut ())
    }
}

impl<T> ProtocolNoCtx for T where T: ProtocolRead + ProtocolWrite {}
