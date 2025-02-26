//! Utilities for externally length prefixed fields

use crate::{BitRead, BitWrite, ByteOrder, Result};

/// A trait for decoding variable-length types with a disjoint length prefix.
pub trait TaggedRead<Tag, Ctx = ()>: Sized {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx, tag: Tag)
        -> Result<Self>;
}

/// A trait for encoding variable-length types with a disjoint length prefix.
pub trait UntaggedWrite<Ctx = ()> {
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()>;
}

macro_rules! test_tagged_read {
    ($ty:ty | $tag:literal: $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn tagged_read() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let read: $ty = $crate::TaggedRead::read(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
                $crate::ByteOrder::BigEndian,
                &mut (),
                $tag,
            )
            .unwrap();
            assert_eq!(exp, read);
        }
    };
}

macro_rules! test_untagged_write {
    ($ty:ty: $value:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn untagged_write() {
            use $crate::UntaggedWrite;

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

macro_rules! test_tagged {
    ($ty:ty | $tag:literal: $value:expr => $bytes:expr) => {
        test_tagged_read!($ty | $tag: $bytes => $value);
        test_untagged_write!($ty: $value => $bytes);
    }
}
