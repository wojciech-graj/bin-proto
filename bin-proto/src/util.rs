//! Helper functions for dealing with sets or lists of parcels.

use crate::{BitRead, BitWrite, ByteOrder, Error, Protocol};

use core::any::Any;
use std::io;

/// Reads a specified number of items from a stream.
pub fn read_items<T>(
    item_count: usize,
    read: &mut dyn BitRead,
    byte_order: ByteOrder,
    ctx: &mut dyn Any,
) -> Result<impl Iterator<Item = T>, Error>
where
    T: Protocol,
{
    let mut elements = Vec::with_capacity(item_count);

    for _ in 0..item_count {
        let element = T::read(read, byte_order, ctx)?;
        elements.push(element);
    }
    Ok(elements.into_iter())
}

/// BitWrites an iterator of parcels to the stream.
///
/// Does not include a length prefix.
pub fn write_items<'a, T>(
    items: impl IntoIterator<Item = &'a T>,
    write: &mut dyn BitWrite,
    byte_order: ByteOrder,
    ctx: &mut dyn Any,
) -> Result<(), Error>
where
    T: Protocol + 'a,
{
    for item in items.into_iter() {
        item.write(write, byte_order, ctx)?;
    }
    Ok(())
}

pub fn read_items_to_eof<T>(
    read: &mut dyn BitRead,
    byte_order: ByteOrder,
    ctx: &mut dyn Any,
) -> Result<Vec<T>, Error>
where
    T: Protocol,
{
    let mut items = Vec::new();
    loop {
        let item = match T::read(read, byte_order, ctx) {
            Ok(item) => item,
            Err(Error::IO(e)) => {
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
