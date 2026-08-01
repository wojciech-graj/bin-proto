macro_rules! impl_newtype {
    ($ty:ident) => {
        impl<E, Ctx, Tag, T> $crate::BitDecode<E, Ctx, Tag> for $ty<T>
        where
            E: ::bitstream_io::Endianness,
            T: $crate::BitDecode<E, Ctx, Tag>,
        {
            fn decode<R>(
                read: &mut R,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead + ?Sized,
            {
                Ok(Self($crate::BitDecode::<E, _, _>::decode(read, ctx, tag)?))
            }
        }

        impl<E, Ctx, Tag, T> $crate::BitEncode<E, Ctx, Tag> for $ty<T>
        where
            E: ::bitstream_io::Endianness,
            T: $crate::BitEncode<E, Ctx, Tag>,
        {
            fn encode<W>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                tag: Tag
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite + ?Sized,
            {
                $crate::BitEncode::<E, _, _>::encode(&self.0, write, ctx, tag)
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

mod reverse {
    use core::cmp::Reverse;

    impl_newtype!(Reverse);
    test_roundtrip!(Reverse<u8>);
}
