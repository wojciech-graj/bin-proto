use crate::{BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result};

macro_rules! impl_tuple {
    ($($idx:tt $t:ident),*) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<Ctx, $($t,)*> $crate::ProtocolRead<Ctx> for ($($t,)*)
        where
            $($t: $crate::ProtocolRead<Ctx>,)*
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<Self> {
                Ok(($(<$t as $crate::ProtocolRead<Ctx>>::read(read, byte_order, ctx)?,)*))
            }
        }

        #[cfg_attr(docsrs, doc(hidden))]
        impl<Ctx, $($t,)*> $crate::ProtocolWrite<Ctx> for ($($t,)*)
        where
            $($t: $crate::ProtocolWrite<Ctx>,)*
        {
            fn write(
                &self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<()> {
                $(
                    $crate::ProtocolWrite::write(&self.$idx, write, byte_order, ctx)?;
                )*
                Ok(())
            }
        }
    };
}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples with up to 16 items."
)]
impl<Ctx, T> ProtocolRead<Ctx> for (T,)
where
    T: ProtocolRead<Ctx>,
{
    fn read(read: &mut dyn BitRead, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<Self> {
        Ok((ProtocolRead::read(read, byte_order, ctx)?,))
    }
}

#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples with up to 16 items."
)]
#[cfg_attr(docsrs, doc(fake_variadic))]
impl<Ctx, T> ProtocolWrite<Ctx> for (T,)
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        self.0.write(write, byte_order, ctx)
    }
}

impl<Ctx> ProtocolRead<Ctx> for () {
    fn read(_: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        Ok(())
    }
}

impl<Ctx> ProtocolWrite<Ctx> for () {
    fn write(&self, _: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K, 10 L, 11 M, 12 N, 13 O, 14 P, 15 Q);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K, 10 L, 11 M, 12 N, 13 O, 14 P);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K, 10 L, 11 M, 12 N, 13 O);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K, 10 L, 11 M, 12 N);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K, 10 L, 11 M);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K, 10 L);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 K);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_tuple!(0 A, 1 B, 2 C, 3 D);
impl_tuple!(0 A, 1 B, 2 C);
impl_tuple!(0 A, 1 B);
