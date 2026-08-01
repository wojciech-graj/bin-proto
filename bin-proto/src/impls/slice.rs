use bitstream_io::{BitWrite, Endianness};

use crate::{util, BitEncode, Result, Untagged};

impl<E, Ctx, T> BitEncode<E, Ctx, Untagged> for [T]
where
    E: Endianness,
    T: BitEncode<E, Ctx>,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, _: Untagged) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        util::encode_items::<_, _, E, _, _>(self.iter(), write, ctx)
    }
}

#[cfg(feature = "prepend-tags")]
impl<E, Ctx, T> BitEncode<E, Ctx> for [T]
where
    E: Endianness,
    T: BitEncode<E, Ctx>,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        BitEncode::<E, _>::encode(&self.len(), write, ctx, ())?;
        self.encode(write, ctx, Untagged)
    }
}

#[cfg(feature = "alloc")]
#[allow(clippy::wildcard_imports)]
mod decode {
    use alloc::{boxed::Box, vec::Vec};
    use bitstream_io::BitRead;

    use crate::BitDecode;

    use super::*;

    impl<E, Ctx, T> BitDecode<E, Ctx, Untagged> for Box<[T]>
    where
        E: Endianness,
        T: BitDecode<E, Ctx>,
    {
        fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: Untagged) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            Vec::decode(read, ctx, tag).map(Into::into)
        }
    }

    impl<E, Ctx, Tag, T> BitDecode<E, Ctx, crate::Tag<Tag>> for Box<[T]>
    where
        E: Endianness,
        T: BitDecode<E, Ctx>,
        Tag: TryInto<usize>,
    {
        fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: crate::Tag<Tag>) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            <Vec<T> as BitDecode<E, _, _>>::decode(read, ctx, tag).map(Into::into)
        }
    }

    #[cfg(feature = "prepend-tags")]
    impl<E, Ctx, T> BitDecode<E, Ctx> for Box<[T]>
    where
        E: Endianness,
        T: BitDecode<E, Ctx>,
    {
        fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            Vec::decode(read, ctx, ()).map(Into::into)
        }
    }

    test_decode!(Box<[u8]>| crate::Tag(3); [0x01, 0x02, 0x03] => Box::new([1, 2, 3]));

    #[cfg(test)]
    mod untagged {
        use super::*;

        test_decode!(Box<[u8]>| Untagged; [0x01, 0x02, 0x03] => Box::new([1, 2, 3]));
    }

    test_length_tag_decode!(Box<[u8]>);

    #[cfg(feature = "prepend-tags")]
    test_roundtrip!(Box<[i32]>);
}

test_encode!(&[u8]| Untagged; &[1, 2, 3] => [0x01, 0x02, 0x03]);
