use core::net::Ipv6Addr;

use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Result};

impl<E, Ctx> BitDecode<E, Ctx> for Ipv6Addr
where
    E: Endianness,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        <u128 as BitDecode<E, _>>::decode(read, ctx, ()).map(Self::from_bits)
    }
}

impl<E, Ctx> BitEncode<E, Ctx> for Ipv6Addr
where
    E: Endianness,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        BitEncode::<E, _>::encode(&self.to_bits(), write, ctx, ())
    }
}

test_codec!(Ipv6Addr;
    Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334) =>
    [
        0x20, 0x01, 0x0d, 0xb8, 0x85, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x2e, 0x03, 0x70, 0x73,
        0x34
    ]
);
test_roundtrip!(Ipv6Addr);
