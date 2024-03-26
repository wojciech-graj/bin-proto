use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use core::any::Any;
use std::convert::TryFrom;

impl Protocol for char {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        let bytes = u32::read(read, settings, ctx)?;
        Ok(char::try_from(bytes)?)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        (*self as u32).write(write, settings, ctx)
    }
}
