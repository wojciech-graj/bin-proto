macro_rules! impl_newtype {
    ($ty:ident) => {
        impl<Ctx, Tag, T> $crate::BitDecode<Ctx, Tag> for $ty<T>
        where
            T: $crate::BitDecode<Ctx, Tag>,
        {
            fn decode<R, E>(
                read: &mut R,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                Ok(Self($crate::BitDecode::decode::<_, E>(read,  ctx, tag)?))
            }
        }

        impl<Ctx, Tag, T> $crate::BitEncode<Ctx, Tag> for $ty<T>
        where
            T: $crate::BitEncode<Ctx, Tag>,
        {
            fn encode<W, E>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                tag: Tag
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness,
            {
                $crate::BitEncode::encode::<_, E>(&self.0, write,  ctx, tag)
            }
        }

        test_codec!($ty<u8>; $ty(1u8) => [0x01]);
    };
}

mod wrapping {
    use core::num::Wrapping;

    impl_newtype!(Wrapping);
    test_roundtrip!(Wrapping<u8>);
}

mod saturating {
    use core::num::Saturating;

    impl_newtype!(Saturating);
    test_roundtrip!(Saturating<u8>);
}
