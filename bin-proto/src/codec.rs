#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter, Endianness};

use crate::{
    error::ErrorCause,
    io::{self, Cursor},
    Error, Result,
};

/// A trait for bit-level decoding.
pub trait BitDecode<E, Ctx = (), Tag = ()>: Sized
where
    E: Endianness,
{
    /// Reads self from a stream.
    fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> Result<Self>
    where
        R: BitRead + ?Sized;
}

/// Utility functionality for bit-level decoding.
pub trait BitDecodeExt {
    /// Parses a new value from its raw byte representation with provided context and tag.
    ///
    /// Returns a tuple of the parsed value and the number of bits read.
    fn decode_bytes_ctx<E, Ctx, Tag>(bytes: &[u8], ctx: &mut Ctx, tag: Tag) -> Result<(Self, u64)>
    where
        Self: BitDecode<E, Ctx, Tag>,
        E: Endianness,
    {
        let mut buffer = BitReader::<_, E>::new(io::Cursor::new(bytes));
        let this = Self::decode(&mut buffer, ctx, tag)?;
        Ok((this, buffer.position_in_bits()?))
    }

    /// Parses a new value from its raw byte representation with provided context and tag, consuming
    /// entire buffer.
    fn decode_all_bytes_ctx<E, Ctx, Tag>(bytes: &[u8], ctx: &mut Ctx, tag: Tag) -> Result<Self>
    where
        Self: BitDecode<E, Ctx, Tag>,
        E: Endianness,
    {
        let (decoded, read_bits) = Self::decode_bytes_ctx(bytes, ctx, tag)?;
        let available_bits = u64::try_from(bytes.len())? * 8;
        if read_bits == available_bits {
            Ok(decoded)
        } else {
            Err(Error::from_inner(ErrorCause::Underrun {
                read_bits,
                available_bits,
            }))
        }
    }

    /// Parses a new value from its raw byte representation.
    ///
    /// Returns a tuple of the parsed value and the number of bits read.
    fn decode_bytes<E>(bytes: &[u8]) -> Result<(Self, u64)>
    where
        Self: BitDecode<E>,
        E: Endianness,
    {
        Self::decode_bytes_ctx::<E, _, _>(bytes, &mut (), ())
    }

    /// Parses a new value from its raw byte representation, consuming entire buffer.
    fn decode_all_bytes<E>(bytes: &[u8]) -> Result<Self>
    where
        Self: BitDecode<E>,
        E: Endianness,
    {
        Self::decode_all_bytes_ctx::<E, _, _>(bytes, &mut (), ())
    }
}

impl<T> BitDecodeExt for T {}

/// A trait for bit-level encoding.
pub trait BitEncode<E, Ctx = (), Tag = ()>
where
    E: Endianness,
{
    /// Writes a value to a stream.
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
    where
        W: BitWrite + ?Sized;
}

/// Utility functionality for bit-level encoding.
pub trait BitEncodeExt {
    /// Gets the raw bytes of this type with provided context and tag.
    #[cfg(feature = "alloc")]
    fn encode_bytes_ctx<E, Ctx, Tag>(&self, ctx: &mut Ctx, tag: Tag) -> Result<Vec<u8>>
    where
        Self: BitEncode<E, Ctx, Tag>,
        E: Endianness,
    {
        let mut data = Vec::new();
        let mut writer = BitWriter::<_, E>::new(&mut data);
        self.encode(&mut writer, ctx, tag)?;
        writer.byte_align()?;

        Ok(data)
    }

    /// Fills the buffer with the raw bytes of this type with provided context and tag.
    ///
    /// Returns the number of bytes written.
    fn encode_bytes_ctx_buf<E, Ctx, Tag>(
        &self,
        buf: &mut [u8],
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<u64>
    where
        Self: BitEncode<E, Ctx, Tag>,
        E: Endianness,
    {
        let mut cursor = Cursor::new(buf);
        let mut writer = BitWriter::<_, E>::new(&mut cursor);
        self.encode(&mut writer, ctx, tag)?;
        writer.byte_align()?;

        Ok(cursor.position())
    }

    /// Gets the raw bytes of this type.
    #[cfg(feature = "alloc")]
    fn encode_bytes<E>(&self) -> Result<Vec<u8>>
    where
        Self: BitEncode<E>,
        E: Endianness,
    {
        self.encode_bytes_ctx(&mut (), ())
    }

    /// Fills the buffer with the raw bytes of this type.
    ///
    /// Returns the number of bytes written.
    fn encode_bytes_buf<E>(&self, buf: &mut [u8]) -> Result<u64>
    where
        Self: BitEncode<E>,
        E: Endianness,
    {
        self.encode_bytes_ctx_buf(buf, &mut (), ())
    }
}

impl<T> BitEncodeExt for T {}

macro_rules! test_decode {
    ($ty:ty | $tag:expr; $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn decode() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let read: $ty = $crate::BitDecodeExt::decode_all_bytes_ctx::<
                ::bitstream_io::BigEndian,
                (),
                _,
            >(bytes, &mut (), $tag)
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
            let decoded: $ty =
                $crate::BitDecodeExt::decode_all_bytes::<::bitstream_io::BigEndian>(bytes).unwrap();
            assert_eq!(exp, decoded);
        }
    };
}

macro_rules! test_encode {
    ($ty:ty $(| $tag:expr)?; $value:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn encode() {
            let exp: &[u8] = &$exp;
            let value: $ty = $value;

            let mut buffer = [0u8; 16];
            $crate::BitEncodeExt::encode_bytes_ctx_buf::<::bitstream_io::BigEndian, _, _>(
                &value,
                &mut buffer,
                &mut (),
                ($($tag)?),
            ).unwrap();
            assert_eq!(exp, &buffer[..exp.len()]);
            assert!(::core::iter::Iterator::all(
                &mut ::core::iter::IntoIterator::into_iter(&buffer[exp.len()..]),
                |x| *x == 0)
            );
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
                let encoded = $crate::BitEncodeExt::encode_bytes::<::bitstream_io::BigEndian>(&x);
                ::proptest::prop_assert!(encoded.is_ok());
                let encoded = encoded.unwrap();
                let decoded = <$ty as $crate::BitDecodeExt>::decode_all_bytes::<::bitstream_io::BigEndian>(&encoded);
                ::proptest::prop_assert!(decoded.is_ok());
                let decoded = decoded.unwrap();
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

#[allow(unused)]
macro_rules! test_length_tag_decode {
    ($ty:ty) => {
        #[cfg(all(test, feature = "alloc"))]
        #[test]
        fn decode_try_reserve() {
            assert_eq!(
                $crate::error::ErrorKind::TryReserve,
                <$ty as $crate::BitDecode::<::bitstream_io::BigEndian, (), _>>::decode(
                    &mut ::bitstream_io::BitReader::<_, ::bitstream_io::BigEndian>::new(
                        [0u8; 0].as_slice()
                    ),
                    &mut (),
                    $crate::Tag(usize::MAX),
                )
                .unwrap_err()
                .kind()
            );
        }
    };
}
