#![cfg(feature = "alloc")]

use alloc::{ffi::CString, vec::Vec};
use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{util, BitDecode, BitEncode, Result};

impl<Ctx> BitDecode<Ctx> for CString {
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
        E: Endianness,
    {
        let mut result = Vec::new();
        loop {
            let c: u8 = BitDecode::decode::<_, E>(read, ctx, ())?;
            if c == 0x00 {
                return Ok(Self::new(result)?);
            }
            result.push(c);
        }
    }
}

impl<Ctx> BitEncode<Ctx> for CString {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
        E: Endianness,
    {
        util::encode_items::<_, _, E, _, _>(self.to_bytes_with_nul().iter(), write, ctx)
    }
}

test_codec!(CString; CString::new("ABC").unwrap() => [0x41, 0x42, 0x43, 0]);
test_roundtrip!(CString);
