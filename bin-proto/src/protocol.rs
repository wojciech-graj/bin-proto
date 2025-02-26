use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::io;

use bitstream_io::{BigEndian, BitReader, BitWriter, LittleEndian};
#[cfg(not(feature = "std"))]
use core2::io;

use crate::{BitRead, BitWrite, ByteOrder, Result};

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

macro_rules! test_protocol_read {
    ($ty:ty: $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn protocol_read() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let read: $ty = $crate::ProtocolRead::read(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
                $crate::ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
            assert_eq!(exp, read);
        }
    };
}

macro_rules! test_protocol_write {
    ($ty:ty: $value:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn protocol_write() {
            use $crate::ProtocolWrite;

            let mut buffer: ::alloc::vec::Vec<u8> = ::alloc::vec::Vec::new();
            let exp: &[u8] = &$exp;
            let value: $ty = $value;
            value
                .write(
                    &mut ::bitstream_io::BitWriter::endian(&mut buffer, ::bitstream_io::BigEndian),
                    $crate::ByteOrder::BigEndian,
                    &mut (),
                )
                .unwrap();
            assert_eq!(exp, &buffer);
        }
    };
}

macro_rules! test_protocol {
    ($ty:ty: $value:expr => $bytes:expr) => {
        test_protocol_read!($ty: $bytes => $value);
        test_protocol_write!($ty: $value => $bytes);
    }
}
