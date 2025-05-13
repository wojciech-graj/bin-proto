//! Helper functions for dealing with iterators

use crate::{BitDecode, BitEncode, Error, Result};

use bitstream_io::{BitRead, BitWrite, Endianness};
use core::iter;
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use core2::io;

/// Reads a specified number of items from a stream.
pub fn decode_items<'a, R, E, Ctx, T>(
    item_count: usize,
    read: &'a mut R,
    ctx: &'a mut Ctx,
) -> impl Iterator<Item = Result<T>> + use<'a, R, E, Ctx, T>
where
    R: BitRead,
    E: Endianness,
    T: BitDecode<Ctx>,
{
    iter::repeat_with(|| T::decode::<_, E>(read, ctx, ())).take(item_count)
}

/// [`BitEncode`]s an iterator of parcels to the stream.
///
/// Does not include a length prefix.
pub fn encode_items<'a, W, E, Ctx, T>(
    items: impl IntoIterator<Item = &'a T>,
    write: &mut W,
    ctx: &mut Ctx,
) -> Result<()>
where
    W: BitWrite,
    E: Endianness,
    T: BitEncode<Ctx> + 'a,
{
    for item in items {
        item.encode::<_, E>(write, ctx, ())?;
    }
    Ok(())
}

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
