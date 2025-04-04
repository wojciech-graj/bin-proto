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
