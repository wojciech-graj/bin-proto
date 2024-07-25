macro_rules! impl_protocol_for_tuple {
    ($($idx:tt $t:tt),*) => {
        impl<Ctx, $($t,)*> $crate::ProtocolRead<Ctx> for ($($t,)*)
        where
            $($t: $crate::ProtocolRead<Ctx>,)*
        {
            #[allow(unused)]
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<Self> {
                Ok(($(<$t as $crate::ProtocolRead<Ctx>>::read(read, byte_order, ctx)?,)*))
            }
        }

        impl<Ctx, $($t,)*> $crate::ProtocolWrite<Ctx> for ($($t,)*)
        where
            $($t: $crate::ProtocolWrite<Ctx>,)*
        {
            #[allow(unused)]
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

impl_protocol_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_protocol_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_protocol_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_protocol_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_protocol_for_tuple!(0 A, 1 B, 2 C, 3 D);
impl_protocol_for_tuple!(0 A, 1 B, 2 C);
impl_protocol_for_tuple!(0 A, 1 B);
impl_protocol_for_tuple!(0 A);
impl_protocol_for_tuple!();
