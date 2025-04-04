use bitstream_io::{BitWrite, Endianness};

use crate::{util, BitEncode, Result};

impl<Ctx, T> BitEncode<Ctx> for [T]
where
    T: BitEncode<Ctx>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.iter(), write, ctx)
    }
}

test_encode!(&[u8]; &[1, 2, 3] => [0x01, 0x02, 0x03]);
