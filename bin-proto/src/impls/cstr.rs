use core::ffi::CStr;

use bitstream_io::{BitWrite, Endianness};

use crate::{util, BitEncode, Result};

impl<Ctx> BitEncode<Ctx> for CStr {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.to_bytes_with_nul().iter(), write, ctx)
    }
}

test_encode!(
    &CStr; CStr::from_bytes_with_nul(&[0x41, 0x42, 0x43, 0]).unwrap() => [0x41, 0x42, 0x43, 0]
);
