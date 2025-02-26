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

#[cfg(test)]
macro_rules! test_externally_tagged {
    ($t:ty => [$bytes:expr, $value:expr]) => {
        #[test]
        fn read_externally_tagged() {
            let bytes: &[u8] = $bytes.as_slice();
            assert_eq!(
                <$t as $crate::TaggedRead<_, _>>::read(
                    &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
                    $crate::ByteOrder::BigEndian,
                    &mut (),
                    $value.len()
                )
                .unwrap(),
                $value
            )
        }

        #[test]
        fn write_externally_tagged() {
            let mut buffer: ::alloc::vec::Vec<u8> = ::alloc::vec::Vec::new();
            let value: $t = $value;
            $crate::UntaggedWrite::<_>::write(
                &value,
                &mut ::bitstream_io::BitWriter::endian(&mut buffer, ::bitstream_io::BigEndian),
                $crate::ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
            assert_eq!(buffer.as_slice(), $bytes)
        }
    };
}
