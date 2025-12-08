use core::net::Ipv4Addr;

use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

impl<Ctx> BitDecode<Ctx> for Ipv4Addr {
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        u32::decode::<_, E>(read, ctx, ()).map(Self::from_bits)
    }
}

impl<Ctx> BitEncode<Ctx> for Ipv4Addr {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        self.to_bits().encode::<_, E>(write, ctx, ())
    }
}

test_codec!(Ipv4Addr; Ipv4Addr::new(192, 168, 1, 0) => [192, 168, 1, 0]);
test_roundtrip!(Ipv4Addr);
