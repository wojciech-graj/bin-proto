macro_rules! impl_newtype {
    ($ty:ident) => {
        impl<Ctx, Tag, T> $crate::ProtocolRead<Ctx, Tag> for $ty<T>
        where
            T: $crate::ProtocolRead<Ctx, Tag>,
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<Self> {
                Ok(Self($crate::ProtocolRead::read(read, byte_order, ctx, tag)?))
            }
        }

        impl<Ctx, Tag, T> $crate::ProtocolWrite<Ctx, Tag> for $ty<T>
        where
            T: $crate::ProtocolWrite<Ctx, Tag>,
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
