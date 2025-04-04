use core::net::Ipv6Addr;

use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

impl<Ctx> BitDecode<Ctx> for Ipv6Addr {
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let bytes: [u16; 8] = BitDecode::decode::<_, E>(read, ctx, tag)?;

        Ok(Self::new(
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ))
    }
}

impl<Ctx> BitEncode<Ctx> for Ipv6Addr {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.octets().encode::<_, E>(write, ctx, ())
    }
}

test_codec!(Ipv6Addr;
    Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334) =>
    [
        0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e, 0x03, 0x70, 0x73,
        0x34
    ]
);
