#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::io::{self, Cursor};

use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter, Endianness};
#[cfg(not(feature = "std"))]
use core2::io::{self, Cursor};

use crate::{Error, Result};

/// A trait for bit-level decoding.
pub trait BitDecode<Ctx = (), Tag = ()>: Sized {
    /// Reads self from a stream.
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> Result<Self>
    where
        R: BitRead,
        E: Endianness;
}

/// Utility functionality for bit-level decoding.
pub trait BitDecodeExt<Ctx = (), Tag = ()>:
    BitDecode<Ctx, Tag> + bit_decode::Sealed<Ctx, Tag>
{
    /// Parses a new value from its raw byte representation with provided context and tag.
    ///
    /// Returns a tuple of the parsed value and the number of bits read.
    fn decode_bytes_ctx<E>(
        bytes: &[u8],
        byte_order: E,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<(Self, u64)>
    where
        E: Endianness,
    {
        let mut buffer = BitReader::endian(io::Cursor::new(bytes), byte_order);
        let this = Self::decode::<_, E>(&mut buffer, ctx, tag)?;
        Ok((this, buffer.position_in_bits()?))
    }

    /// Parses a new value from its raw byte representation with provided context and tag, consuming
    /// entire buffer.
    fn decode_all_bytes_ctx<E>(bytes: &[u8], byte_order: E, ctx: &mut Ctx, tag: Tag) -> Result<Self>
    where
        E: Endianness,
    {
        let (decoded, read_bits) = Self::decode_bytes_ctx(bytes, byte_order, ctx, tag)?;
        let available_bits = u64::try_from(bytes.len())? * 8;
        if read_bits == available_bits {
            Ok(decoded)
        } else {
            Err(Error::Underrun {
                read_bits,
                available_bits,
            })
        }
    }
}

impl<T, Ctx, Tag> BitDecodeExt<Ctx, Tag> for T where
    T: BitDecode<Ctx, Tag> + bit_decode::Sealed<Ctx, Tag>
{
}

/// A trait for bit-level encoding.
pub trait BitEncode<Ctx = (), Tag = ()> {
    /// Writes a value to a stream.
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
    where
        W: BitWrite,
        E: Endianness;
}

/// Utility functionality for bit-level encoding.
pub trait BitEncodeExt<Ctx = (), Tag = ()>:
    BitEncode<Ctx, Tag> + bit_encode::Sealed<Ctx, Tag>
{
    /// Gets the raw bytes of this type with provided context and tag.
    #[cfg(feature = "alloc")]
    fn encode_bytes_ctx<E>(&self, byte_order: E, ctx: &mut Ctx, tag: Tag) -> Result<Vec<u8>>
    where
        E: Endianness,
    {
        let mut data = Vec::new();
        let mut writer = BitWriter::endian(&mut data, byte_order);
        self.encode::<_, E>(&mut writer, ctx, tag)?;
        writer.byte_align()?;

        Ok(data)
    }

    /// Fills the buffer with the raw bytes of this type with provided context and tag.
    ///
    /// Returns the number of bytes written.
    fn encode_bytes_ctx_buf<E>(
        &self,
        byte_order: E,
        ctx: &mut Ctx,
        tag: Tag,
        buf: &mut [u8],
    ) -> Result<u64>
    where
        E: Endianness,
    {
        let mut cursor = Cursor::new(buf);
        let mut writer = BitWriter::endian(&mut cursor, byte_order);
        self.encode::<_, E>(&mut writer, ctx, tag)?;
        writer.byte_align()?;

        Ok(cursor.position())
    }
}

impl<T, Ctx, Tag> BitEncodeExt<Ctx, Tag> for T where
    T: BitEncode<Ctx, Tag> + bit_encode::Sealed<Ctx, Tag>
{
}

/// A trait with helper functions for simple codecs.
pub trait BitCodec: BitDecode + BitEncode + bit_codec::Sealed {
    /// Parses a new value from its raw byte representation.
    ///
    /// Returns a tuple of the parsed value and the number of bits read.
    fn decode_bytes<E>(bytes: &[u8], byte_order: E) -> Result<(Self, u64)>
    where
        E: Endianness,
    {
        Self::decode_bytes_ctx(bytes, byte_order, &mut (), ())
    }

    /// Parses a new value from its raw byte representation, consuming entire buffer.
    fn decode_all_bytes<E>(bytes: &[u8], byte_order: E) -> Result<Self>
    where
        E: Endianness,
    {
        Self::decode_all_bytes_ctx(bytes, byte_order, &mut (), ())
    }

    /// Gets the raw bytes of this type.
    #[cfg(feature = "alloc")]
    fn encode_bytes<E>(&self, byte_order: E) -> Result<Vec<u8>>
    where
        E: Endianness,
    {
        self.encode_bytes_ctx(byte_order, &mut (), ())
    }

    /// Fills the buffer with the raw bytes of this type.
    ///
    /// Returns the number of bytes written.
    fn encode_bytes_buf<E>(&self, byte_order: E, buf: &mut [u8]) -> Result<u64>
    where
        E: Endianness,
    {
        self.encode_bytes_ctx_buf(byte_order, &mut (), (), buf)
    }
}

impl<T> BitCodec for T where T: BitDecode + BitEncode + bit_codec::Sealed {}

mod bit_encode {
    use super::BitEncode;

    pub trait Sealed<Ctx, Tag> {}

    impl<Ctx, Tag, T> Sealed<Ctx, Tag> for T where T: BitEncode<Ctx, Tag> {}
}

mod bit_decode {
    use super::BitDecode;

    pub trait Sealed<Ctx, Tag> {}

    impl<Ctx, Tag, T> Sealed<Ctx, Tag> for T where T: BitDecode<Ctx, Tag> {}
}

mod bit_codec {
    use super::{BitDecode, BitEncode};

    pub trait Sealed {}

    impl<T> Sealed for T where T: BitDecode + BitEncode {}
}

macro_rules! test_decode {
    ($ty:ty | $tag:expr; $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn decode() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let read: $ty = $crate::BitDecode::<(), _>::decode::<_, ::bitstream_io::BigEndian>(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
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
            let decoded: $ty = $crate::BitDecode::decode::<_, ::bitstream_io::BigEndian>(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
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

            let exp: &[u8] = &$exp;
            let value: $ty = $value;

            #[cfg(feature = "alloc")]
            {
                let mut buffer: ::alloc::vec::Vec<u8> = ::alloc::vec::Vec::new();
                value
                    .encode::<_, ::bitstream_io::BigEndian>(
                        &mut ::bitstream_io::BitWriter::endian(&mut buffer, ::bitstream_io::BigEndian),
                        &mut (),
                        ($($tag)?),
                    )
                    .unwrap();
                assert_eq!(exp, &buffer);
            }

            #[cfg(not(feature = "alloc"))]
            {
                let mut buffer = [0u8; 16];
                value
                    .encode::<_, ::bitstream_io::BigEndian>(
                        &mut ::bitstream_io::BitWriter::endian(&mut ::core2::io::Cursor::new(buffer.as_mut_slice()), ::bitstream_io::BigEndian),
                        &mut (),
                        ($($tag)?),
                    )
                    .unwrap();
                assert_eq!(exp, &buffer[..exp.len()]);
                assert!(::core::iter::Iterator::all(
                    &mut ::core::iter::IntoIterator::into_iter(&buffer[exp.len()..]),
                    |x| *x == 0)
                );
            }
        }
    };
}

macro_rules! test_codec {
    ($ty:ty$(| $tag_write:expr, $tag_read:expr)?; $value:expr => $bytes:expr) => {
        test_decode!($ty$(| $tag_read)?; $bytes => $value);
        test_encode!($ty$(| $tag_write)?; $value => $bytes);
    }
}

macro_rules! test_roundtrip {
    ($ty:ty) => {
        #[cfg(all(test, feature = "std"))]
        ::proptest::proptest!(
            #[test]
            fn roundtrip(x in ::proptest::arbitrary::any::<$ty>()) {
                let encoded = $crate::BitEncodeExt::encode_bytes_ctx(&x, ::bitstream_io::BigEndian, &mut (), ()).unwrap();
                let decoded = <$ty as $crate::BitDecodeExt>::decode_bytes_ctx(&encoded, ::bitstream_io::BigEndian, &mut (), ()).unwrap().0;
                ::proptest::prop_assert_eq!(x, decoded);
            }
        );
    }
}

#[allow(unused)]
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
