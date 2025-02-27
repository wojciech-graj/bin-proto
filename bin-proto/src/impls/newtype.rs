macro_rules! impl_newtype {
    ($ty:ident) => {
        impl<Ctx, T> $crate::ProtocolRead<Ctx> for $ty<T>
        where
            T: $crate::ProtocolRead<Ctx>,
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: (),
            ) -> $crate::Result<Self> {
                Ok(Self($crate::ProtocolRead::read(read, byte_order, ctx, tag)?))
            }
        }

        impl<Ctx, T> $crate::ProtocolWrite<Ctx> for $ty<T>
        where
            T: $crate::ProtocolWrite<Ctx>,
        {
            fn write(
                &self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<()> {
                $crate::ProtocolWrite::write(&self.0, write, byte_order, ctx)
            }
        }

        test_protocol!($ty<u8>: $ty(1u8) => [0x01]);
    };
}

mod wrapping {
    use core::num::Wrapping;

    impl_newtype!(Wrapping);
}

mod saturating {
    use core::num::Saturating;

    impl_newtype!(Saturating);
}
