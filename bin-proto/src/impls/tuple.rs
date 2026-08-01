use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

macro_rules! impl_tuple {
    ($($idx:tt $t:ident),*) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<E, Ctx, $($t,)*> $crate::BitDecode<E, Ctx> for ($($t,)*)
        where
            E: ::bitstream_io::Endianness,
            $($t: $crate::BitDecode<E, Ctx>,)*
        {
            fn decode<R>(
                read: &mut R,
                ctx: &mut Ctx,
                (): (),
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead + ?Sized,
            {
                Ok(($(<$t as $crate::BitDecode<E, Ctx>>::decode(read,  ctx, ())?,)*))
            }
        }

        #[cfg_attr(docsrs, doc(hidden))]
        impl<E, Ctx, $($t,)*> $crate::BitEncode<E, Ctx> for ($($t,)*)
        where
            E: ::bitstream_io::Endianness,
            $($t: $crate::BitEncode<E, Ctx>,)*
        {
            fn encode<W>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                (): ()
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite + ?Sized,
            {
                $(
                    $crate::BitEncode::<E, _>::encode(&self.$idx, write, ctx, ())?;
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
impl<E, Ctx, Tag, T> BitDecode<E, Ctx, Tag> for (T,)
where
    E: Endianness,
    T: BitDecode<E, Ctx, Tag>,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        Ok((BitDecode::decode(read, ctx, tag)?,))
    }
}

#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples with up to 16 items."
)]
#[cfg_attr(docsrs, doc(fake_variadic))]
impl<E, Ctx, Tag, T> BitEncode<E, Ctx, Tag> for (T,)
where
    E: Endianness,
    T: BitEncode<E, Ctx, Tag> + ?Sized,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        self.0.encode(write, ctx, tag)
    }
}

impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14, 15 T15);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
impl_tuple!(0 T0, 1 T1, 2 T2, 3 T3);
impl_tuple!(0 T0, 1 T1, 2 T2);
impl_tuple!(0 T0, 1 T1);

test_codec!((u8,); (1,) => [0x01]);
test_roundtrip!((u8,));
