//! Utilities for conditional fields

use crate::{BitRead, BitWrite, ByteOrder, Error};

/// A trait for Option types that are read/written conditionally.
pub trait Conditional<Ctx = ()>: Sized {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        condition: bool,
    ) -> Result<Self, Error>;

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
    ) -> Result<(), Error>;
}

#[cfg(test)]
macro_rules! test_conditional {
    ($t:ty => [$bytes:expr, $value:expr]) => {
        #[test]
        fn read_conditional() {
            let bytes: &[u8] = $bytes.as_slice();
            assert_eq!(
                <$t as crate::Conditional>::read(
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
        fn write_conditional() {
            let mut buffer: Vec<u8> = Vec::new();
            let value: $t = $value;
            crate::Conditional::write(
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
