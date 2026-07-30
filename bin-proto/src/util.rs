//! Helper functions for dealing with iterators

use bitstream_io::{BitRead, BitWrite, Endianness};
use core::iter;

use crate::{io, BitDecode, BitEncode, Error, Result};

/// [`BitEncode`]s an iterator.
///
/// Does not include a length prefix.
pub fn encode_items<I, W, E, Ctx, T>(items: I, write: &mut W, ctx: &mut Ctx) -> Result<()>
where
    I: IntoIterator<Item = T>,
    W: BitWrite,
    E: Endianness,
    T: BitEncode<Ctx>,
{
    for item in items {
        item.encode::<_, E>(write, ctx, ())?;
    }
    Ok(())
}

/// [`BitDecode`]s items until EOF
pub fn decode_items_to_eof<'a, R, E, Ctx, T>(
    read: &'a mut R,
    ctx: &'a mut Ctx,
) -> impl Iterator<Item = Result<T>> + use<'a, R, E, Ctx, T>
where
    R: BitRead,
    E: Endianness,
    T: BitDecode<Ctx>,
{
    iter::from_fn(|| match T::decode::<_, E>(read, ctx, ()) {
        Err(Error::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof => None,
        other => Some(other),
    })
}
