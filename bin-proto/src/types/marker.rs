use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};
use core::marker::{PhantomData, PhantomPinned};

impl<Ctx, T> ProtocolRead<Ctx> for PhantomData<T> {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        Ok(Self)
    }
}

impl<Ctx, T> ProtocolWrite<Ctx> for PhantomData<T> {
    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

impl<Ctx> ProtocolRead<Ctx> for PhantomPinned {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        Ok(Self)
    }
}

impl<Ctx> ProtocolWrite<Ctx> for PhantomPinned {
    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ProtocolNoCtx;

    use super::*;

    #[test]
    fn can_read_phantom_data() {
        assert_eq!(
            PhantomData::<u8>::from_bytes(&[], ByteOrder::BigEndian).unwrap(),
            PhantomData
        )
    }

    #[test]
    fn can_write_phantom_data() {
        assert_eq!(PhantomData::<u8>.bytes(ByteOrder::BigEndian).unwrap(), &[])
    }

    #[test]
    fn can_read_phantom_pinned() {
        assert_eq!(
            PhantomPinned::from_bytes(&[], ByteOrder::BigEndian).unwrap(),
            PhantomPinned
        )
    }

    #[test]
    fn can_write_phantom_pinned() {
        assert_eq!(PhantomPinned.bytes(ByteOrder::BigEndian).unwrap(), &[])
    }
}
