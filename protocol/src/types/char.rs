use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};
use std::convert::TryFrom;

impl Parcel for char {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let bytes = u32::read(read, settings)?;
        Ok(char::try_from(bytes)?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        (*self as u32).write(write, settings)
    }
}
