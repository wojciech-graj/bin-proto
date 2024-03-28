use crate::{BitRead, BitWrite, ByteOrder, Error, Protocol};
use core::any::Any;

impl Protocol for () {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut dyn Any) -> Result<Self, Error> {
        Ok(())
    }

    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut dyn Any) -> Result<(), Error> {
        Ok(())
    }
}

impl<T0> Protocol for (T0,)
where
    T0: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v = T0::read(read, byte_order, ctx)?;
        Ok((v,))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1> Protocol for (T0, T1)
where
    T0: Protocol,
    T1: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        Ok((v0, v1))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2> Protocol for (T0, T1, T2)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3> Protocol for (T0, T1, T2, T3)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4> Protocol for (T0, T1, T2, T3, T4)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5> Protocol for (T0, T1, T2, T3, T4, T5)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
    T5: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        let v5 = T5::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4, v5))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;
        self.5.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Protocol for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
    T5: Protocol,
    T6: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        let v5 = T5::read(read, byte_order, ctx)?;
        let v6 = T6::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4, v5, v6))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;
        self.5.write(write, byte_order, ctx)?;
        self.6.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Protocol for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
    T5: Protocol,
    T6: Protocol,
    T7: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        let v5 = T5::read(read, byte_order, ctx)?;
        let v6 = T6::read(read, byte_order, ctx)?;
        let v7 = T7::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;
        self.5.write(write, byte_order, ctx)?;
        self.6.write(write, byte_order, ctx)?;
        self.7.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Protocol for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
    T5: Protocol,
    T6: Protocol,
    T7: Protocol,
    T8: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        let v5 = T5::read(read, byte_order, ctx)?;
        let v6 = T6::read(read, byte_order, ctx)?;
        let v7 = T7::read(read, byte_order, ctx)?;
        let v8 = T8::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;
        self.5.write(write, byte_order, ctx)?;
        self.6.write(write, byte_order, ctx)?;
        self.7.write(write, byte_order, ctx)?;
        self.8.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Protocol for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
    T5: Protocol,
    T6: Protocol,
    T7: Protocol,
    T8: Protocol,
    T9: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        let v5 = T5::read(read, byte_order, ctx)?;
        let v6 = T6::read(read, byte_order, ctx)?;
        let v7 = T7::read(read, byte_order, ctx)?;
        let v8 = T8::read(read, byte_order, ctx)?;
        let v9 = T9::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;
        self.5.write(write, byte_order, ctx)?;
        self.6.write(write, byte_order, ctx)?;
        self.7.write(write, byte_order, ctx)?;
        self.8.write(write, byte_order, ctx)?;
        self.9.write(write, byte_order, ctx)?;

        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Protocol
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Protocol,
    T1: Protocol,
    T2: Protocol,
    T3: Protocol,
    T4: Protocol,
    T5: Protocol,
    T6: Protocol,
    T7: Protocol,
    T8: Protocol,
    T9: Protocol,
    T10: Protocol,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<Self, Error> {
        let v0 = T0::read(read, byte_order, ctx)?;
        let v1 = T1::read(read, byte_order, ctx)?;
        let v2 = T2::read(read, byte_order, ctx)?;
        let v3 = T3::read(read, byte_order, ctx)?;
        let v4 = T4::read(read, byte_order, ctx)?;
        let v5 = T5::read(read, byte_order, ctx)?;
        let v6 = T6::read(read, byte_order, ctx)?;
        let v7 = T7::read(read, byte_order, ctx)?;
        let v8 = T8::read(read, byte_order, ctx)?;
        let v9 = T9::read(read, byte_order, ctx)?;
        let v10 = T10::read(read, byte_order, ctx)?;
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10))
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        self.0.write(write, byte_order, ctx)?;
        self.1.write(write, byte_order, ctx)?;
        self.2.write(write, byte_order, ctx)?;
        self.3.write(write, byte_order, ctx)?;
        self.4.write(write, byte_order, ctx)?;
        self.5.write(write, byte_order, ctx)?;
        self.6.write(write, byte_order, ctx)?;
        self.7.write(write, byte_order, ctx)?;
        self.8.write(write, byte_order, ctx)?;
        self.9.write(write, byte_order, ctx)?;
        self.10.write(write, byte_order, ctx)?;

        Ok(())
    }
}
