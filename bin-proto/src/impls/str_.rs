use crate::{util, BitEncode, Result};

use bitstream_io::{BitWrite, Endianness};

impl<Ctx> BitEncode<Ctx> for str {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.as_bytes(), write, ctx)
    }
}

#[cfg(feature = "alloc")]
mod decode {
    use alloc::{boxed::Box, string::String};
    use bitstream_io::BitRead;

    use crate::{BitDecode, Untagged};

    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<Ctx> BitDecode<Ctx, Untagged> for Box<str> {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Untagged) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            String::decode::<_, E>(read, ctx, tag).map(Into::into)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<Tag, Ctx> BitDecode<Ctx, crate::Tag<Tag>> for Box<str>
    where
        Tag: TryInto<usize>,
    {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            String::decode::<_, E>(read, ctx, tag).map(Into::into)
        }
    }

    test_decode!(Box<str>| crate::Tag(3); [b'a', b'b', b'c'] => "abc".into());

    #[cfg(test)]
    mod untagged {
        use super::*;

        test_decode!(Box<str>| Untagged; [b'a', b'b', b'c'] => "abc".into());
    }
}

test_encode!(&str; "abc" => [b'a', b'b', b'c']);
