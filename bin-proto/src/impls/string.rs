use crate::{util, BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Error, Result, Untagged};

use alloc::{string::String, vec::Vec};

impl<Tag, Ctx> BitDecode<Ctx, crate::Tag<Tag>> for String
where
    Tag: TryInto<usize>,
{
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: crate::Tag<Tag>,
    ) -> Result<Self> {
        let bytes = util::decode_items(
            tag.0.try_into().map_err(|_| Error::TagConvert)?,
            read,
            byte_order,
            ctx,
        )?;
        Ok(Self::from_utf8(bytes)?)
    }
}

impl<Ctx> BitEncode<Ctx, Untagged> for String {
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        _: Untagged,
    ) -> Result<()> {
        let bytes: Vec<_> = self.bytes().collect();
        util::encode_items(&bytes, write, byte_order, ctx)
    }
}

impl<Ctx> BitDecode<Ctx, Untagged> for String {
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        _: Untagged,
    ) -> Result<Self> {
        let bytes = util::decode_items_to_eof(read, byte_order, ctx)?;
        Ok(Self::from_utf8(bytes)?)
    }
}

test_untagged_and_codec!(String| Untagged, crate::Tag(3); "abc".into() => [b'a', b'b', b'c']);
