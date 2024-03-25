use crate::{BitRead, BitWrite, Error, Protocol, Settings};

use uuid::Uuid;

impl Protocol for Uuid {
    fn read(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        let bytes: [u8; 16] = Protocol::read(read, settings)?;

        Ok(Uuid::from_bytes(bytes))
    }

    fn write(&self, write: &mut dyn BitWrite, _: &Settings) -> Result<(), Error> {
        write.write_bytes(self.as_bytes())?;
        Ok(())
    }
}
