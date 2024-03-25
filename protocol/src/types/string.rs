use crate::{hint, util, BitRead, BitWrite, Error, Parcel, Settings, WithLengthPrefix};

// The default implementation treats the string as a normal char array.
impl Parcel for String {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list_nohint(read, settings)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        util::write_list_nohint(&bytes, write, settings)
    }
}

impl WithLengthPrefix for String {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let bytes: Vec<u8> = util::read_list(read, settings, hints)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        let bytes: Vec<u8> = self.bytes().collect();
        util::write_list(&bytes, write, settings)
    }
}
