use crate::{BitRead, ByteOrder, Result};

/// A trait for variable-length types without a length prefix.
pub trait FlexibleArrayMemberRead<Ctx = ()>: Sized {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self>;
}

macro_rules! test_flexible_array_member_read {
    ($ty:ty: $bytes:expr => $exp:expr) => {
        #[cfg(test)]
        #[test]
        fn flexible_array_member_read() {
            let bytes: &[u8] = &$bytes;
            let exp: $ty = $exp;
            let read: $ty = $crate::FlexibleArrayMemberRead::read(
                &mut ::bitstream_io::BitReader::endian(bytes, ::bitstream_io::BigEndian),
                $crate::ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
            assert_eq!(exp, read);
        }
    };
}

macro_rules! test_flexible_array_member_read_and_tagged {
    ($ty:ty | $tag:literal: $value:expr => $bytes:expr) => {
        test_tagged_read!($ty | $tag: $bytes => $value);
        test_untagged_write!($ty: $value => $bytes);
        test_flexible_array_member_read!($ty: $bytes => $value);
    }
}
