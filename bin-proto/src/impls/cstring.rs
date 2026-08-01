#![cfg(feature = "alloc")]

use alloc::{ffi::CString, vec::Vec};
use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{util, BitDecode, BitEncode, Result};

impl<E, Ctx> BitDecode<E, Ctx> for CString
where
    E: Endianness,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        let mut result = Vec::new();
        loop {
            let c: u8 = BitDecode::<E, _>::decode(read, ctx, ())?;
            if c == 0x00 {
                return Ok(Self::new(result)?);
            }
            result.push(c);
        }
    }
}

impl<E, Ctx> BitEncode<E, Ctx> for CString
where
    E: Endianness,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        util::encode_items::<_, _, E, _, _>(self.to_bytes_with_nul().iter(), write, ctx)
    }
}

test_codec!(CString; CString::new("ABC").unwrap() => [0x41, 0x42, 0x43, 0]);
test_roundtrip!(CString);
