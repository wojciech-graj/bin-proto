use crate::{BitRead, BitWrite, Error, Parcel, Settings};

impl<T> Parcel for std::ops::Range<T>
where
    T: Parcel,
{
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let start = Parcel::read(read, settings)?;
        let end = Parcel::read(read, settings)?;

        Ok(std::ops::Range { start, end })
    }

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        self.start.write(write, settings)?;
        self.end.write(write, settings)?;

        Ok(())
    }
}
