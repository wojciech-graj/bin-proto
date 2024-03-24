use crate::{hint, BitRead, BitWrite, CharTryFromError, Error, Parcel, Settings};
use std::char;

impl Parcel for char {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let bytes = u32::read(read, settings)?;
        Ok(char::from_u32(bytes).ok_or(CharTryFromError {})?)
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
