use crate::{util, BitRead, BitWrite, Error, ExternallyLengthPrefixed, Settings};
use core::any::Any;

impl ExternallyLengthPrefixed for String {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        length: usize,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_items(length, read, settings, ctx)?.collect();

        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_list(&bytes, write, settings, ctx)
    }
}

#[cfg(test)]
mod tests {
    test_externally_length_prefixed!(String => [[b'a', b'b', b'c', b'd'], String::from("abcd")]);
}
