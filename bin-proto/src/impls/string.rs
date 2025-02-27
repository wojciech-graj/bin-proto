use crate::{
    util, BitRead, BitWrite, ByteOrder, Error, ProtocolRead, ProtocolWrite, Result, Untagged,
};

use alloc::{string::String, vec::Vec};

impl<Tag, Ctx> ProtocolRead<Ctx, crate::Tag<Tag>> for String
where
    Tag: TryInto<usize>,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: crate::Tag<Tag>,
    ) -> Result<Self> {
        let bytes = util::read_items(
            tag.0.try_into().map_err(|_| Error::TagConvert)?,
            read,
            byte_order,
            ctx,
        )?;
        Ok(Self::from_utf8(bytes)?)
    }
}

impl<Ctx> ProtocolWrite<Ctx, Untagged> for String {
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        _: Untagged,
    ) -> Result<()> {
        let bytes: Vec<_> = self.bytes().collect();
        util::write_items(&bytes, write, byte_order, ctx)
    }
}

impl<Ctx> ProtocolRead<Ctx, Untagged> for String {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        _: Untagged,
    ) -> Result<Self> {
        let bytes = util::read_items_to_eof(read, byte_order, ctx)?;
        Ok(Self::from_utf8(bytes)?)
    }
}

test_untagged_and_protocol!(String| Untagged, crate::Tag(3); "abc".into() => [b'a', b'b', b'c']);
