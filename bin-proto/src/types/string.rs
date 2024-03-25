use crate::{
    externally_length_prefixed, util, BitRead, BitWrite, Error, ExternallyLengthPrefixed, Protocol,
    Settings,
};

// The default implementation treats the string as a normal char array.
impl Protocol for String {
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list(read, settings)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        util::write_list_length_prefixed(&bytes, write, settings)
    }
}

impl ExternallyLengthPrefixed for String {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut externally_length_prefixed::Hints,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list_with_hints(read, settings, hints)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut externally_length_prefixed::Hints,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        util::write_list(&bytes, write, settings)
    }
}
