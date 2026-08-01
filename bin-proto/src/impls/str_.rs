use crate::{util, BitEncode, Result, Untagged};

use bitstream_io::{BitWrite, Endianness};

impl<E, Ctx> BitEncode<E, Ctx, Untagged> for str
where
    E: Endianness,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, _: Untagged) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        util::encode_items::<_, _, E, _, _>(self.as_bytes(), write, ctx)
    }
}

#[cfg(feature = "prepend-tags")]
impl<E, Ctx> BitEncode<E, Ctx> for str
where
    E: Endianness,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        BitEncode::<E, _>::encode(&self.len(), write, ctx, ())?;
        BitEncode::<E, _, _>::encode(&self, write, ctx, Untagged)
    }
}

#[cfg(feature = "alloc")]
#[allow(clippy::wildcard_imports)]
mod decode {
    use alloc::{boxed::Box, string::String};
    use bitstream_io::BitRead;

    use crate::BitDecode;

    use super::*;

    impl<E, Ctx> BitDecode<E, Ctx, Untagged> for Box<str>
    where
        E: Endianness,
    {
        fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: Untagged) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            <String as BitDecode<E, _, _>>::decode(read, ctx, tag).map(Into::into)
        }
    }

    impl<E, Ctx, Tag> BitDecode<E, Ctx, crate::Tag<Tag>> for Box<str>
    where
        E: Endianness,
        Tag: TryInto<usize>,
    {
        fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            <String as BitDecode<E, _, _>>::decode(read, ctx, tag).map(Into::into)
        }
    }

    #[cfg(feature = "prepend-tags")]
    impl<E, Ctx> BitDecode<E, Ctx> for Box<str>
    where
        E: Endianness,
    {
        fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            <String as BitDecode<E, _>>::decode(read, ctx, ()).map(Into::into)
        }
    }

    test_decode!(Box<str>| crate::Tag(3); [b'a', b'b', b'c'] => "abc".into());

    #[cfg(test)]
    mod untagged {
        use super::*;

        test_decode!(Box<str>| Untagged; [b'a', b'b', b'c'] => "abc".into());
    }

    test_length_tag_decode!(Box<str>);

    #[cfg(feature = "prepend-tags")]
    test_roundtrip!(Box<str>);
}

test_encode!(&str| Untagged; "abc" => [b'a', b'b', b'c']);
