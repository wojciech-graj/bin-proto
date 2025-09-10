use crate::{util, BitEncode, Result};

use bitstream_io::{BitWrite, Endianness};

impl<Ctx> BitEncode<Ctx> for str {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.as_bytes(), write, ctx)
    }
}

test_encode!(&str; "abc" => [b'a', b'b', b'c']);
