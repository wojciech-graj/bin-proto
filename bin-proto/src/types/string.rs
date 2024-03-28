use crate::{
    util, BitRead, BitWrite, ByteOrder, Error, ExternallyLengthPrefixed, FlexibleArrayMember,
};
use core::any::Any;

impl ExternallyLengthPrefixed for String {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
        length: usize,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_items(length, read, byte_order, ctx)?.collect();

        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_items(&bytes, write, byte_order, ctx)
    }
}

impl FlexibleArrayMember for String {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_items_to_eof(read, byte_order, ctx)?;
        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_items(&bytes, write, byte_order, ctx)
    }
}

#[cfg(test)]
mod tests {
    test_externally_length_prefixed!(String => [[b'a', b'b', b'c', b'd'], String::from("abcd")]);
}
