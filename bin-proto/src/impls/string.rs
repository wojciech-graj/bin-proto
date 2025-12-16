#![cfg(feature = "alloc")]

use crate::{util, BitDecode, BitEncode, Error, Result, Untagged};

use alloc::string::String;
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
        let bytes = util::decode_items::<_, E, _, _>(
            tag.0.try_into().map_err(|_| Error::TagConvert)?,
            read,
            ctx,
        )
        .collect::<Result<_>>()?;
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

test_untagged_and_codec!(String| Untagged, crate::Tag(3); "abc".into() => [b'a', b'b', b'c']);
