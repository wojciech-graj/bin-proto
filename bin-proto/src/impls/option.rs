use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Error, Result, Untagged};

impl<Tag, Ctx, T> BitDecode<Ctx, crate::Tag<Tag>> for Option<T>
where
    T: BitDecode<Ctx>,
    Tag: TryInto<bool>,
{
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: crate::Tag<Tag>,
    ) -> Result<Self> {
        if tag.0.try_into().map_err(|_| Error::TagConvert)? {
            let value = T::decode(read, byte_order, ctx, ())?;
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
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        _: Untagged,
    ) -> Result<()> {
        if let Some(ref value) = *self {
            value.encode(write, byte_order, ctx, ())?;
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
