use crate::{
    externally_length_prefixed::FieldLength, util, BitRead, BitWrite, Error,
    ExternallyLengthPrefixed, Protocol, Settings,
};
use core::any::Any;

// The default implementation treats the string as a normal char array.
impl Protocol for String {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list(read, settings, ctx)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_list_length_prefixed(&bytes, write, settings, ctx)
    }
}

impl ExternallyLengthPrefixed for String {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        length: &FieldLength,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list_with_hints(read, settings, ctx, length)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
        _: &FieldLength,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = str::bytes(self).collect();
        util::write_list(&bytes, write, settings, ctx)
    }
}
