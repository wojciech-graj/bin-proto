use crate::{
    util, BitRead, BitWrite, ByteOrder, Error, ExternallyLengthPrefixed, FlexibleArrayMember,
};

impl<Ctx> ExternallyLengthPrefixed<Ctx> for String {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        length: usize,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_items(length, read, byte_order, ctx)?.collect();

        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_items::<Ctx, u8>(&bytes, write, byte_order, ctx)
    }
}

impl<Ctx> FlexibleArrayMember<Ctx> for String {
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_items_to_eof(read, byte_order, ctx)?;
        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_items::<Ctx, u8>(&bytes, write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    test_externally_length_prefixed!(String => [[b'a', b'b', b'c', b'd'], String::from("abcd")]);
}
