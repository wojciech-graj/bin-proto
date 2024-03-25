use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use std::convert::TryFrom;

impl Protocol for char {
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let bytes = u32::read(read, settings)?;
        Ok(char::try_from(bytes)?)
    }

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        (*self as u32).write(write, settings)
    }
}
