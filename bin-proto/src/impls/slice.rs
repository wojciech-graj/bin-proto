use bitstream_io::{BitWrite, Endianness};

use crate::{util, BitEncode, Result};

impl<Ctx, T> BitEncode<Ctx> for [T]
where
    T: BitEncode<Ctx>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.iter(), write, ctx)
    }
}

#[cfg(feature = "alloc")]
#[allow(clippy::wildcard_imports)]
mod decode {
    use alloc::{boxed::Box, vec::Vec};
    use bitstream_io::BitRead;

    use crate::{BitDecode, Untagged};

    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<Ctx, T> BitDecode<Ctx, Untagged> for Box<[T]>
    where
        T: BitDecode<Ctx>,
    {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Untagged) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            Vec::decode::<_, E>(read, ctx, tag).map(Into::into)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<Tag, Ctx, T> BitDecode<Ctx, crate::Tag<Tag>> for Box<[T]>
    where
        T: BitDecode<Ctx>,
        Tag: TryInto<usize>,
    {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            Vec::decode::<_, E>(read, ctx, tag).map(Into::into)
        }
    }

    test_decode!(Box<[u8]>| crate::Tag(3); [0x01, 0x02, 0x03] => Box::new([1, 2, 3]));

    #[cfg(test)]
    mod untagged {
        use super::*;

        test_decode!(Box<[u8]>| Untagged; [0x01, 0x02, 0x03] => Box::new([1, 2, 3]));
    }
}

test_encode!(&[u8]; &[1, 2, 3] => [0x01, 0x02, 0x03]);
