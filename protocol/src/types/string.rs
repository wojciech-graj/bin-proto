use crate::{hint, util, BitRead, BitWrite, Error, Parcel, Settings};

// The default implementation treats the string as a normal char array.
impl Parcel for std::string::String {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list(read, settings, hints)?;

        Ok(std::string::String::from_utf8(bytes)?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        util::write_list(&bytes, write, settings, hints)
    }
}
