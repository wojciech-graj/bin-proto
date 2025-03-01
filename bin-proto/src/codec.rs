use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::io;

use bitstream_io::{BigEndian, BitReader, BitWriter, LittleEndian};
#[cfg(not(feature = "std"))]
use core2::io;

use crate::{BitRead, BitWrite, ByteOrder, Result};

/// A trait for bit-level decoding.
pub trait BitDecode<Ctx = (), Tag = ()>: Sized {
    /// Reads self from a stream.
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<Self>;

    /// Parses a new value from its raw byte representation with provided context and tag.
    fn decode_bytes_ctx(
        bytes: &[u8],
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<Self> {
        match byte_order {
            ByteOrder::LittleEndian => {
                let mut buffer = BitReader::endian(io::Cursor::new(bytes), LittleEndian);
                Self::decode(&mut buffer, byte_order, ctx, tag)
            }
            ByteOrder::BigEndian => {
                let mut buffer = BitReader::endian(io::Cursor::new(bytes), BigEndian);
                Self::decode(&mut buffer, byte_order, ctx, tag)
            }
        }
    }
}

/// A trait for bit-level encoding.
pub trait BitEncode<Ctx = (), Tag = ()> {
    /// Writes a value to a stream.
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<()>;

    /// Gets the raw bytes of this type with provided context and tag.
    fn encode_bytes_ctx(&self, byte_order: ByteOrder, ctx: &mut Ctx, tag: Tag) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        match byte_order {
            ByteOrder::LittleEndian => {
                let mut writer = BitWriter::endian(&mut data, LittleEndian);
                self.encode(&mut writer, byte_order, ctx, tag)?;
                writer.byte_align()?;
            }
            ByteOrder::BigEndian => {
                let mut writer = BitWriter::endian(&mut data, BigEndian);
                self.encode(&mut writer, byte_order, ctx, tag)?;
                writer.byte_align()?;
            }
        };

        Ok(data)
    }
}

/// A trait with helper functions for simple codecs
pub trait BitCodec: BitDecode + BitEncode {
    /// Parses a new value from its raw byte representation without context.
    fn decode_bytes(bytes: &[u8], byte_order: ByteOrder) -> Result<Self> {
        Self::decode_bytes_ctx(bytes, byte_order, &mut (), ())
    }

    /// Gets the raw bytes of this type without context.
    fn encode_bytes(&self, byte_order: ByteOrder) -> Result<Vec<u8>> {
        self.encode_bytes_ctx(byte_order, &mut (), ())
    }
}

impl<T> BitCodec for T where T: BitDecode + BitEncode {}

macro_rules! test_decode {
    ($ty:ty | $tag:expr; $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn decode() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let read: $ty = $crate::BitDecode::<(), _>::decode(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
                $crate::ByteOrder::BigEndian,
                &mut (),
                $tag,
            )
            .unwrap();
            assert_eq!(exp, read);
        }
    };
    ($ty:ty; $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn decode() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let decoded: $ty = $crate::BitDecode::decode(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
                $crate::ByteOrder::BigEndian,
                &mut (),
                (),
            )
            .unwrap();
            assert_eq!(exp, decoded);
        }
    };
}

macro_rules! test_encode {
    ($ty:ty $(| $tag:expr)?; $value:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn encode() {
            use $crate::BitEncode;

            let mut buffer: ::alloc::vec::Vec<u8> = ::alloc::vec::Vec::new();
            let exp: &[u8] = &$exp;
            let value: $ty = $value;
            value
                .encode(
                    &mut ::bitstream_io::BitWriter::endian(&mut buffer, ::bitstream_io::BigEndian),
                    $crate::ByteOrder::BigEndian,
                    &mut (),
                    ($($tag)?),
                )
                .unwrap();
            assert_eq!(exp, &buffer);
        }
    };
}

macro_rules! test_codec {
    ($ty:ty$(| $tag_write:expr, $tag_read:expr)?; $value:expr => $bytes:expr) => {
        test_decode!($ty$(| $tag_read)?; $bytes => $value);
        test_encode!($ty$(| $tag_write)?; $value => $bytes);
    }
}

macro_rules! test_untagged_and_codec {
    ($ty:ty | $tag_write:expr, $tag_read:expr; $value:expr => $bytes:expr) => {
        test_codec!($ty | $tag_write, $tag_read; $value => $bytes);
        #[cfg(test)]
        mod untagged {
            use super::*;

            test_decode!($ty| $crate::Untagged; $bytes => $value);
        }
    }
}
