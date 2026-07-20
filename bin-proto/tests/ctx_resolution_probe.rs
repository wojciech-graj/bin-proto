#![cfg(all(feature = "derive", feature = "alloc"))]

use bin_proto::{BitDecode, BitDecodeExt, BitEncode, BitEncodeExt};
use bitstream_io::{BigEndian, BitRead, BitWrite, Endianness};

struct CustomCtx;

/// Writes 0xAA with `()` ctx, 0xBB with `CustomCtx`.
#[derive(Debug, PartialEq, Eq)]
struct CtxSensitive;

impl BitEncode<()> for CtxSensitive {
    fn encode<W: BitWrite, E: Endianness>(
        &self,
        w: &mut W,
        _: &mut (),
        (): (),
    ) -> Result<(), bin_proto::Error> {
        BitEncode::<(), ()>::encode::<_, E>(&0xAAu8, w, &mut (), ())
    }
}

impl BitEncode<CustomCtx> for CtxSensitive {
    fn encode<W: BitWrite, E: Endianness>(
        &self,
        w: &mut W,
        _: &mut CustomCtx,
        (): (),
    ) -> Result<(), bin_proto::Error> {
        BitEncode::<(), ()>::encode::<_, E>(&0xBBu8, w, &mut (), ())
    }
}

impl BitDecode<()> for CtxSensitive {
    fn decode<R: BitRead, E: Endianness>(
        _: &mut R,
        _: &mut (),
        (): (),
    ) -> Result<Self, bin_proto::Error> {
        Ok(Self)
    }
}

impl BitDecode<CustomCtx> for CtxSensitive {
    fn decode<R: BitRead, E: Endianness>(
        _: &mut R,
        _: &mut CustomCtx,
        (): (),
    ) -> Result<Self, bin_proto::Error> {
        Ok(Self)
    }
}

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
struct Pre<T: BitEncode + BitDecode> {
    field: T,
}

#[test]
fn generic_impl_uses_generic_ctx_not_unit_ctx() {
    // Encoding with CustomCtx must dispatch to BitEncode<CustomCtx> (0xBB),
    // not BitEncode<()> (0xAA).
    let bytes = Pre {
        field: CtxSensitive,
    }
    .encode_bytes_ctx(BigEndian, &mut CustomCtx, ())
    .unwrap();
    assert_eq!(vec![0xBB], bytes);

    // And with the unit ctx it must dispatch to BitEncode<()>.
    let bytes = Pre {
        field: CtxSensitive,
    }
    .encode_bytes_ctx(BigEndian, &mut (), ())
    .unwrap();
    assert_eq!(vec![0xAA], bytes);

    let _ = Pre::<CtxSensitive>::decode_bytes_ctx(&[0xBB], BigEndian, &mut CustomCtx, ()).unwrap();
}
