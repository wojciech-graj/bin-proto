use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{error::ErrorCause, BitDecode, BitEncode, Error, Result, Untagged};

impl<E, Tag, Ctx, T> BitDecode<E, Ctx, crate::Tag<Tag>> for Option<T>
where
    E: Endianness,
    T: BitDecode<E, Ctx>,
    Tag: TryInto<bool>,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        if tag
            .0
            .try_into()
            .map_err(|_| Error::from_inner(ErrorCause::TagConvert))?
        {
            let value = T::decode(read, ctx, ())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<E, Ctx, T> BitEncode<E, Ctx, Untagged> for Option<T>
where
    E: Endianness,
    T: BitEncode<E, Ctx>,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, _: Untagged) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        if let Some(ref value) = *self {
            value.encode(write, ctx, ())?;
        }
        Ok(())
    }
}

#[cfg(feature = "prepend-tags")]
impl<E, Ctx, T> BitEncode<E, Ctx> for Option<T>
where
    E: Endianness,
    T: BitEncode<E, Ctx>,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        BitEncode::<E, _>::encode(&self.is_some(), write, ctx, ())?;
        self.encode(write, ctx, Untagged)
    }
}

#[cfg(feature = "prepend-tags")]
impl<E, Ctx, T> BitDecode<E, Ctx> for Option<T>
where
    E: Endianness,
    T: BitDecode<E, Ctx>,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        let tag: bool = BitDecode::<E, _>::decode(read, ctx, ())?;
        Self::decode(read, ctx, crate::Tag(tag))
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
