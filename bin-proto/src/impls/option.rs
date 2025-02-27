use crate::{BitRead, BitWrite, ByteOrder, Error, ProtocolRead, ProtocolWrite, Result, Untagged};

impl<Tag, Ctx, T> ProtocolRead<Ctx, crate::Tag<Tag>> for Option<T>
where
    T: ProtocolRead<Ctx>,
    Tag: TryInto<bool>,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: crate::Tag<Tag>,
    ) -> Result<Self> {
        if tag.0.try_into().map_err(|_| Error::TagConvert)? {
            let value = T::read(read, byte_order, ctx, ())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Ctx, T> ProtocolWrite<Ctx, Untagged> for Option<T>
where
    T: ProtocolWrite<Ctx>,
{
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        _: Untagged,
    ) -> Result<()> {
        if let Some(ref value) = *self {
            value.write(write, byte_order, ctx, ())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod none {
    use crate::Tag;

    use super::*;

    test_protocol!(Option<u8>| Untagged, Tag(false); None => []);
}

#[cfg(test)]
mod some {
    use crate::Tag;

    use super::*;

    test_protocol!(Option<u8>| Untagged, Tag(true); Some(1) => [0x01]);
}
