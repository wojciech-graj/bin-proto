//! Helper functions for dealing with sets or lists of parcels.

use crate::{BitRead, BitWrite, Error, Protocol, Settings};

use core::any::Any;
use std::io;

/// Reads a specified number of items from a stream.
pub fn read_items<T>(
    item_count: usize,
    read: &mut dyn BitRead,
    settings: &Settings,
    ctx: &mut dyn Any,
) -> Result<impl Iterator<Item = T>, Error>
where
    T: Protocol,
{
    let mut elements = Vec::with_capacity(item_count);

    for _ in 0..item_count {
        let element = T::read(read, settings, ctx)?;
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
    settings: &Settings,
    ctx: &mut dyn Any,
) -> Result<(), Error>
where
    T: Protocol + 'a,
{
    for item in items.into_iter() {
        item.write(write, settings, ctx)?;
    }
    Ok(())
}

pub fn read_list_to_eof<T>(
    read: &mut dyn BitRead,
    settings: &Settings,
    ctx: &mut dyn Any,
) -> Result<Vec<T>, Error>
where
    T: Protocol,
{
    let mut items = Vec::new();
    loop {
        let item = match T::read(read, settings, ctx) {
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

/// BitWrites a length-prefixed list to a stream.
pub fn write_list<'a, T, I>(
    elements: I,
    write: &mut dyn BitWrite,
    settings: &Settings,
    ctx: &mut dyn Any,
) -> Result<(), Error>
where
    T: Protocol + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let elements: Vec<_> = elements.into_iter().collect();

    write_items(elements.into_iter(), write, settings, ctx)?;

    Ok(())
}
