use bitstream_io::{BitWrite, Endianness};

use crate::{BitEncode, Result};

impl<E, Ctx, Tag, T> BitEncode<E, Ctx, Tag> for &mut T
where
    E: Endianness,
    T: BitEncode<E, Ctx, Tag> + ?Sized,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        (**self).encode(write, ctx, tag)
    }
}

test_encode!(&mut u8; &mut 1 => [0x01]);
