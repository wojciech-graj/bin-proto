use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use core::any::Any;

impl<T> Protocol for std::ops::Range<T>
where
    T: Protocol,
{
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        let start = Protocol::read(read, settings, ctx)?;
        let end = Protocol::read(read, settings, ctx)?;

        Ok(std::ops::Range { start, end })
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.start.write(write, settings, ctx)?;
        self.end.write(write, settings, ctx)?;

        Ok(())
    }
}
