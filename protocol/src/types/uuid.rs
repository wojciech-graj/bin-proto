use crate::{BitRead, BitWrite, Error, Parcel, Settings};

use uuid::Uuid;

impl Parcel for Uuid {
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let bytes: [u8; 16] = Parcel::read(read, settings)?;

        Ok(Uuid::from_bytes(bytes))
    }

    fn write_field(&self, write: &mut dyn BitWrite, _: &Settings) -> Result<(), Error> {
        write.write_bytes(self.as_bytes())?;
        Ok(())
    }
}
