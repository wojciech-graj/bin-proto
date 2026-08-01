#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{BitDecode, BitDecodeExt, BitEncode, BitEncodeExt, LittleEndian};
use bitstream_io::{BigEndian, BitRead, BitWrite, Endianness};

struct CustomCtx;

/// Writes 0xAA with `()` ctx, 0xBB with `CustomCtx`.
#[derive(Debug, PartialEq, Eq)]
struct CtxSensitive;

impl<E> BitEncode<E, ()> for CtxSensitive
where
    E: Endianness,
{
    fn encode<W: BitWrite + ?Sized>(
        &self,
        w: &mut W,
        _: &mut (),
        (): (),
    ) -> Result<(), bin_proto::Error> {
        BitEncode::<E, (), ()>::encode(&0xAAu8, w, &mut (), ())
    }
}

impl<E> BitEncode<E, CustomCtx> for CtxSensitive
where
    E: Endianness,
{
    fn encode<W: BitWrite + ?Sized>(
        &self,
        w: &mut W,
        _: &mut CustomCtx,
        (): (),
    ) -> Result<(), bin_proto::Error> {
        BitEncode::<E, (), ()>::encode(&0xBBu8, w, &mut (), ())
    }
}

impl<E> BitDecode<E, ()> for CtxSensitive
where
    E: Endianness,
{
    fn decode<R: BitRead + ?Sized>(
        _: &mut R,
        _: &mut (),
        (): (),
    ) -> Result<Self, bin_proto::Error> {
        Ok(Self)
    }
}

impl<E> BitDecode<E, CustomCtx> for CtxSensitive
where
    E: Endianness,
{
    fn decode<R: BitRead + ?Sized>(
        _: &mut R,
        _: &mut CustomCtx,
        (): (),
    ) -> Result<Self, bin_proto::Error> {
        Ok(Self)
    }
}

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
struct Pre<T: BitEncode<LittleEndian> + BitDecode<LittleEndian>> {
    field: T,
}

#[test]
fn generic_impl_uses_generic_ctx_not_unit_ctx() {
    // Encoding with CustomCtx must dispatch to BitEncode<CustomCtx> (0xBB),
    // not BitEncode<()> (0xAA).
    let bytes = Pre {
        field: CtxSensitive,
    }
    .encode_bytes_ctx::<BigEndian, _, _>(&mut CustomCtx, ())
    .unwrap();
    assert_eq!(vec![0xBB], bytes);

    // And with the unit ctx it must dispatch to BitEncode<()>.
    let bytes = Pre {
        field: CtxSensitive,
    }
    .encode_bytes_ctx::<BigEndian, _, _>(&mut (), ())
    .unwrap();
    assert_eq!(vec![0xAA], bytes);

    let _ = Pre::<CtxSensitive>::decode_bytes_ctx::<BigEndian, _, _>(&[0xBB], &mut CustomCtx, ())
        .unwrap();
}
