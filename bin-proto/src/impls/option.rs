use crate::{BitRead, BitWrite, ByteOrder, Error, ProtocolRead, ProtocolWrite, Result, Untagged};

impl<Tg, Ctx, T> ProtocolRead<Ctx, (Tg,)> for Option<T>
where
    T: ProtocolRead<Ctx>,
    Tg: TryInto<bool>,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: (Tg,),
    ) -> Result<Self> {
        if tag.0.try_into().map_err(|_| Error::TagConvert)? {
            let value = T::read(read, byte_order, ctx, ())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Ctx, T> ProtocolWrite<Ctx, Untagged> for Option<T>
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        if let Some(ref value) = *self {
            value.write(write, byte_order, ctx)?;
        }
        Ok(())
    }
}

mod none {
    test_protocol!(Option<u8>| false: None => []);
}

mod some {
    test_protocol!(Option<u8>| true: Some(1) => [0x01]);
}
