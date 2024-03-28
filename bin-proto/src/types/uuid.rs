use crate::{BitRead, BitWrite, ByteOrder, Error, Protocol};
use core::any::Any;

use uuid::Uuid;

impl Protocol for Uuid {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let bytes: [u8; 16] = Protocol::read(read, byte_order, ctx)?;

        Ok(Uuid::from_bytes(bytes))
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut dyn Any) -> Result<(), Error> {
        write.write_bytes(self.as_bytes())?;
        Ok(())
    }
}
