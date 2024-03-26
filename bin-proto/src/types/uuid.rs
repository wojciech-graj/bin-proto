use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use core::any::Any;

use uuid::Uuid;

impl Protocol for Uuid {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        let bytes: [u8; 16] = Protocol::read(read, settings, ctx)?;

        Ok(Uuid::from_bytes(bytes))
    }

    fn write(&self, write: &mut dyn BitWrite, _: &Settings, _: &mut dyn Any) -> Result<(), Error> {
        write.write_bytes(self.as_bytes())?;
        Ok(())
    }
}
