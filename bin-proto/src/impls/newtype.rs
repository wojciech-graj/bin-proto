macro_rules! impl_newtype {
    ($ty:ident) => {
        impl<Ctx, Tag, T> $crate::BitDecode<Ctx, Tag> for $ty<T>
        where
            T: $crate::BitDecode<Ctx, Tag>,
        {
            fn decode(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<Self> {
                Ok(Self($crate::BitDecode::decode(read, byte_order, ctx, tag)?))
            }
        }

        impl<Ctx, Tag, T> $crate::BitEncode<Ctx, Tag> for $ty<T>
        where
            T: $crate::BitEncode<Ctx, Tag>,
        {
            fn encode(
                &self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: Tag
            ) -> $crate::Result<()> {
                $crate::BitEncode::encode(&self.0, write, byte_order, ctx, tag)
            }
        }

        test_codec!($ty<u8>; $ty(1u8) => [0x01]);
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
