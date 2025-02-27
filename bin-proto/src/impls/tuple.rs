use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Result};

macro_rules! impl_tuple {
    ($($idx:tt $t:ident),*) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<Ctx, $($t,)*> $crate::BitDecode<Ctx> for ($($t,)*)
        where
            $($t: $crate::BitDecode<Ctx>,)*
        {
            fn decode(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                (): (),
            ) -> $crate::Result<Self> {
                Ok(($(<$t as $crate::BitDecode<Ctx>>::decode(read, byte_order, ctx, ())?,)*))
            }
        }

        #[cfg_attr(docsrs, doc(hidden))]
        impl<Ctx, $($t,)*> $crate::BitEncode<Ctx> for ($($t,)*)
        where
            $($t: $crate::BitEncode<Ctx>,)*
        {
            fn encode(
                &self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                (): ()
            ) -> $crate::Result<()> {
                $(
                    $crate::BitEncode::encode(&self.$idx, write, byte_order, ctx, ())?;
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
    fn decode(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<Self> {
        Ok((BitDecode::decode(read, byte_order, ctx, tag)?,))
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
    fn encode(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<()> {
        self.0.encode(write, byte_order, ctx, tag)
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

test_codec!((u8,); (1,) => [0x01]);
