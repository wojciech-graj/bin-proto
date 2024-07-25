//! Utilities for externally length prefixed fields

use crate::{BitRead, BitWrite, ByteOrder, Result};

/// A trait for variable-length types with a disjoint length prefix.
pub trait ExternallyLengthPrefixed<Ctx = ()>: Sized {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        length: usize,
    ) -> Result<Self>;

    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()>;
}

#[cfg(test)]
macro_rules! test_externally_length_prefixed {
    ($t:ty => [$bytes:expr, $value:expr]) => {
        #[test]
        fn read_externally_length_prefixed() {
            let bytes: &[u8] = $bytes.as_slice();
            assert_eq!(
                <$t as crate::ExternallyLengthPrefixed>::read(
                    &mut bitstream_io::BitReader::endian(bytes, bitstream_io::BigEndian),
                    crate::ByteOrder::BigEndian,
                    &mut (),
                    $value.len()
                )
                .unwrap(),
                $value
            )
        }

        #[test]
        fn write_externally_length_prefixed() {
            let mut buffer: Vec<u8> = Vec::new();
            let value: $t = $value;
            crate::ExternallyLengthPrefixed::write(
                &value,
                &mut bitstream_io::BitWriter::endian(&mut buffer, bitstream_io::BigEndian),
                crate::ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
            assert_eq!(buffer.as_slice(), $bytes)
        }
    };
}
