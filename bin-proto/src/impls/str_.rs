use crate::{util, BitEncode, Result, Untagged};

use bitstream_io::{BitWrite, Endianness};

impl<Ctx> BitEncode<Ctx, Untagged> for str {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, _: Untagged) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.as_bytes(), write, ctx)
    }
}

#[cfg(feature = "prepend-tags")]
impl<Ctx> BitEncode<Ctx> for str {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.len().encode::<_, E>(write, ctx, ())?;
        self.encode::<_, E>(write, ctx, Untagged)
    }
}

#[cfg(feature = "alloc")]
#[allow(clippy::wildcard_imports)]
mod decode {
    use alloc::{boxed::Box, string::String};
    use bitstream_io::BitRead;

    use crate::BitDecode;

    use super::*;

    impl<Ctx> BitDecode<Ctx, Untagged> for Box<str> {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Untagged) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            String::decode::<_, E>(read, ctx, tag).map(Into::into)
        }
    }

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

    #[cfg(feature = "prepend-tags")]
    impl<Ctx> BitDecode<Ctx> for Box<str> {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            String::decode::<_, E>(read, ctx, ()).map(Into::into)
        }
    }

    test_decode!(Box<str>| crate::Tag(3); [b'a', b'b', b'c'] => "abc".into());

    #[cfg(test)]
    mod untagged {
        use super::*;

        test_decode!(Box<str>| Untagged; [b'a', b'b', b'c'] => "abc".into());
    }
}

test_encode!(&str| Untagged; "abc" => [b'a', b'b', b'c']);
