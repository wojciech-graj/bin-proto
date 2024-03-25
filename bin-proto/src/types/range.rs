use crate::{BitRead, BitWrite, Error, Protocol, Settings};

impl<T> Protocol for std::ops::Range<T>
where
    T: Protocol,
{
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let start = Protocol::read(read, settings)?;
        let end = Protocol::read(read, settings)?;

        Ok(std::ops::Range { start, end })
    }

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        self.start.write(write, settings)?;
        self.end.write(write, settings)?;

        Ok(())
    }
}
