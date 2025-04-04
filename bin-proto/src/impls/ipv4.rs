use core::net::Ipv4Addr;

use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

impl<Ctx> BitDecode<Ctx> for Ipv4Addr {
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let bytes: [u8; 4] = BitDecode::decode::<_, E>(read, ctx, tag)?;

        Ok(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

impl<Ctx> BitEncode<Ctx> for Ipv4Addr {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.octets().encode::<_, E>(write, ctx, ())
    }
}

test_codec!(Ipv4Addr; Ipv4Addr::new(192, 168, 1, 0) => [192, 168, 1, 0]);
