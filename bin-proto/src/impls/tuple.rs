use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

macro_rules! impl_tuple {
    ($($idx:tt $t:ident),*) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<Ctx, $($t,)*> $crate::BitDecode<Ctx> for ($($t,)*)
        where
            $($t: $crate::BitDecode<Ctx>,)*
        {
            fn decode<R, E>(
                read: &mut R,
                ctx: &mut Ctx,
                (): (),
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                Ok(($(<$t as $crate::BitDecode<Ctx>>::decode::<_, E>(read,  ctx, ())?,)*))
            }
        }

        #[cfg_attr(docsrs, doc(hidden))]
        impl<Ctx, $($t,)*> $crate::BitEncode<Ctx> for ($($t,)*)
        where
            $($t: $crate::BitEncode<Ctx>,)*
        {
            fn encode<W, E>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                (): ()
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness,
            {
                $(
                    $crate::BitEncode::encode::<_, E>(&self.$idx, write,  ctx, ())?;
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
impl<Ctx, Tag, T> BitDecode<Ctx, Tag> for (T,)
where
    T: BitDecode<Ctx, Tag>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        Ok((BitDecode::decode::<R, E>(read, ctx, tag)?,))
    }
}

#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples with up to 16 items."
)]
#[cfg_attr(docsrs, doc(fake_variadic))]
impl<Ctx, Tag, T> BitEncode<Ctx, Tag> for (T,)
where
    T: BitEncode<Ctx, Tag>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.0.encode::<W, E>(write, ctx, tag)
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
