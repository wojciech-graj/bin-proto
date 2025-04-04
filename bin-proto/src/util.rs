//! Helper functions for dealing with iterators

use crate::{BitDecode, BitEncode, Error, Result};

use alloc::vec::Vec;
use bitstream_io::{BitRead, BitWrite, Endianness};
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use core2::io;

/// Reads a specified number of items from a stream.
pub fn decode_items<R, E, Ctx, T>(item_count: usize, read: &mut R, ctx: &mut Ctx) -> Result<Vec<T>>
where
    R: BitRead,
    E: Endianness,
    T: BitDecode<Ctx>,
{
    let mut elements = Vec::with_capacity(item_count);
    for _ in 0..item_count {
        let element = T::decode::<_, E>(read, ctx, ())?;
        elements.push(element);
    }
    Ok(elements)
}

/// [`BitWrite`]s an iterator of parcels to the stream.
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

pub fn decode_items_to_eof<R, E, Ctx, T>(read: &mut R, ctx: &mut Ctx) -> Result<Vec<T>>
where
    R: BitRead,
    E: Endianness,
    T: BitDecode<Ctx>,
{
    let mut items = Vec::new();
    loop {
        let item = match T::decode::<_, E>(read, ctx, ()) {
            Ok(item) => item,
            Err(Error::Io(e)) => {
                return if e.kind() == io::ErrorKind::UnexpectedEof {
                    Ok(items)
                } else {
                    Err(e.into())
                }
            }
            Err(e) => return Err(e),
        };
        items.push(item);
    }
}
