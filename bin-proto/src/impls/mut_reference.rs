use bitstream_io::{BitWrite, Endianness};

use crate::{BitEncode, Result};

impl<Ctx, Tag, T> BitEncode<Ctx, Tag> for &mut T
where
    T: BitEncode<Ctx, Tag>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        (**self).encode::<_, E>(write, ctx, tag)
    }
}

test_encode!(&mut u8; &mut 1 => [0x01]);
