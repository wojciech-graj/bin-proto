//! Helper functions for dealing with sets or lists of parcels.

use crate::{BitDecode, BitEncode, BitRead, BitWrite, ByteOrder, Error, Result};

use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use core2::io;

/// Reads a specified number of items from a stream.
pub fn decode_items<Ctx, T>(
    item_count: usize,
    read: &mut dyn BitRead,
    byte_order: ByteOrder,
    ctx: &mut Ctx,
) -> Result<Vec<T>>
where
    T: BitDecode<Ctx>,
{
    let mut elements = Vec::with_capacity(item_count);
    for _ in 0..item_count {
        let element = T::decode(read, byte_order, ctx, ())?;
        elements.push(element);
    }
    Ok(elements)
}

/// `BitWrites` an iterator of parcels to the stream.
///
/// Does not include a length prefix.
pub fn encode_items<'a, Ctx, T>(
    items: impl IntoIterator<Item = &'a T>,
    write: &mut dyn BitWrite,
    byte_order: ByteOrder,
    ctx: &mut Ctx,
) -> Result<()>
where
    T: BitEncode<Ctx> + 'a,
{
    for item in items {
        item.encode(write, byte_order, ctx, ())?;
    }
    Ok(())
}

pub fn decode_items_to_eof<Ctx, T>(
    read: &mut dyn BitRead,
    byte_order: ByteOrder,
    ctx: &mut Ctx,
) -> Result<Vec<T>>
where
    T: BitDecode<Ctx>,
{
    let mut items = Vec::new();
    loop {
        let item = match T::decode(read, byte_order, ctx, ()) {
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
