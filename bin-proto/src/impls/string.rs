#![cfg(feature = "alloc")]

use crate::{util, BitDecode, BitEncode, Error, Result, Untagged};

use alloc::{string::String, vec::Vec};
use bitstream_io::{BitRead, BitWrite, Endianness};

impl<Tag, Ctx> BitDecode<Ctx, crate::Tag<Tag>> for String
where
    Tag: TryInto<usize>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let item_count = tag.0.try_into().map_err(|_| Error::TagConvert)?;
        let mut bytes = Vec::with_capacity(item_count);
        for _ in 0..item_count {
            bytes.push(u8::decode::<_, E>(read, ctx, ())?);
        }
        Ok(Self::from_utf8(bytes)?)
    }
}

impl<Ctx> BitEncode<Ctx, Untagged> for String {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, _: Untagged) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.as_bytes(), write, ctx)
    }
}

impl<Ctx> BitDecode<Ctx, Untagged> for String {
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, _: Untagged) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let bytes = util::decode_items_to_eof::<_, E, _, _>(read, ctx).collect::<Result<_>>()?;
        Ok(Self::from_utf8(bytes)?)
    }
}

#[cfg(feature = "prepend-tags")]
impl<Ctx> BitEncode<Ctx> for String {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.len().encode::<_, E>(write, ctx, ())?;
        self.encode::<_, E>(write, ctx, Untagged)
    }
}

#[cfg(feature = "prepend-tags")]
impl<Ctx> BitDecode<Ctx> for String {
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let tag = usize::decode::<_, E>(read, ctx, ())?;
        Self::decode::<_, E>(read, ctx, crate::Tag(tag))
    }
}

test_untagged_and_codec!(String| Untagged, crate::Tag(3); "abc".into() => [b'a', b'b', b'c']);

#[cfg(feature = "prepend-tags")]
test_roundtrip!(String);
