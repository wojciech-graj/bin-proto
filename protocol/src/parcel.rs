use bitstream_io::{BigEndian, BitReader, BitWriter};

use crate::{BitRead, BitWrite, Error, Settings};
use std::io;

/// A value which can be read and written.
///
/// All of the expected standard Rust types implement this.
pub trait Parcel: Sized {
    /// Reads a new item with a fresh set of hints.
    ///
    /// Blocks until a value is received.
    fn read(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        Self::read_field(read, settings)
    }

    /// Reads a value from a stream.
    ///
    /// Blocks until a value is received.
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error>;

    /// BitWrites a value to a stream.
    fn write(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        self.write_field(write, settings)
    }

    /// BitWrites a value to a stream.
    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error>;

    /// Convers the value into a byte stream that implements `std::io::Read`.
    fn into_stream(self, settings: &Settings) -> Result<io::Cursor<Vec<u8>>, Error> {
        self.raw_bytes(settings).map(io::Cursor::new)
    }

    /// Parses a new value from its raw byte representation.
    ///
    /// Returns `Err` if the bytes represent an invalid value.
    fn from_raw_bytes(bytes: &[u8], settings: &Settings) -> Result<Self, Error> {
        Self::field_from_raw_bytes(bytes, settings)
    }

    /// Parses a new value from its raw byte representation.
    ///
    /// Returns `Err` if the bytes represent an invalid value.
    fn field_from_raw_bytes(bytes: &[u8], settings: &Settings) -> Result<Self, Error> {
        let mut buffer = BitReader::endian(io::Cursor::new(bytes), BigEndian);
        Self::read_field(&mut buffer, settings)
    }

    /// Gets the raw byte representation of the value.
    fn raw_bytes(&self, settings: &Settings) -> Result<Vec<u8>, Error> {
        self.raw_bytes_field(settings)
    }

    /// Gets the raw bytes of this type as a field of a larger type.
    fn raw_bytes_field(&self, settings: &Settings) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        let mut writer = BitWriter::endian(&mut data, BigEndian);
        self.write_field(&mut writer, settings)?;
        writer.byte_align()?;

        Ok(data)
    }
}
