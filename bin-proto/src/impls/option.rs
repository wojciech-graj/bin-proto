use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Error, Result, Untagged};

impl<Tag, Ctx, T> BitDecode<Ctx, crate::Tag<Tag>> for Option<T>
where
    T: BitDecode<Ctx>,
    Tag: TryInto<bool>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        if tag.0.try_into().map_err(|_| Error::TagConvert)? {
            let value = T::decode::<_, E>(read, ctx, ())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Ctx, T> BitEncode<Ctx, Untagged> for Option<T>
where
    T: BitEncode<Ctx>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, _: Untagged) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        if let Some(ref value) = *self {
            value.encode::<_, E>(write, ctx, ())?;
        }
        Ok(())
    }
}

#[cfg(feature = "prepend-tags")]
impl<Ctx, T> BitEncode<Ctx> for Option<T>
where
    T: BitEncode<Ctx>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.is_some().encode::<_, E>(write, ctx, ())?;
        self.encode::<_, E>(write, ctx, Untagged)
    }
}

#[cfg(feature = "prepend-tags")]
impl<Ctx, T> BitDecode<Ctx> for Option<T>
where
    T: BitDecode<Ctx>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let tag = bool::decode::<_, E>(read, ctx, ())?;
        Self::decode::<_, E>(read, ctx, crate::Tag(tag))
    }
}

#[cfg(test)]
mod none {
    use crate::Tag;

    use super::*;

    test_codec!(Option<u8>| Untagged, Tag(false); None => []);
}

#[cfg(test)]
mod some {
    use crate::Tag;

    use super::*;

    test_codec!(Option<u8>| Untagged, Tag(true); Some(1) => [0x01]);
}

#[cfg(feature = "prepend-tags")]
test_roundtrip!(Option::<i32>);
