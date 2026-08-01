#![cfg(feature = "alloc")]

use crate::{error::ErrorCause, util, BitDecode, BitEncode, Error, Result, Untagged};

use alloc::{string::String, vec::Vec};
use bitstream_io::{BitRead, BitWrite, Endianness};

impl<E, Tag, Ctx> BitDecode<E, Ctx, crate::Tag<Tag>> for String
where
    E: Endianness,
    Tag: TryInto<usize>,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        let item_count = tag
            .0
            .try_into()
            .map_err(|_| Error::from_inner(ErrorCause::TagConvert))?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(item_count)?;
        for _ in 0..item_count {
            bytes.push(<u8 as BitDecode<E, _>>::decode(read, ctx, ())?);
        }
        Ok(Self::from_utf8(bytes)?)
    }
}

impl<E, Ctx> BitEncode<E, Ctx, Untagged> for String
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

impl<E, Ctx> BitDecode<E, Ctx, Untagged> for String
where
    E: Endianness,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, _: Untagged) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        let bytes = util::decode_items_to_eof::<_, E, _, _>(read, ctx).collect::<Result<_>>()?;
        Ok(Self::from_utf8(bytes)?)
    }
}

#[cfg(feature = "prepend-tags")]
impl<E, Ctx> BitEncode<E, Ctx> for String
where
    E: Endianness,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        BitEncode::<E, _>::encode(&self.len(), write, ctx, ())?;
        BitEncode::<E, _, _>::encode(self, write, ctx, Untagged)
    }
}

#[cfg(feature = "prepend-tags")]
impl<E, Ctx> BitDecode<E, Ctx> for String
where
    E: Endianness,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        let tag: usize = BitDecode::<E, _>::decode(read, ctx, ())?;
        BitDecode::<E, _, _>::decode(read, ctx, crate::Tag(tag))
    }
}

test_untagged_and_codec!(String| Untagged, crate::Tag(3); "abc".into() => [b'a', b'b', b'c']);

test_length_tag_decode!(String);

#[cfg(feature = "prepend-tags")]
test_roundtrip!(String);
