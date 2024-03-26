//! Helper functions for dealing with sets or lists of parcels.

use bitstream_io::{BigEndian, BitReader};

use crate::externally_length_prefixed::{FieldLength, LengthPrefixKind};
use crate::{BitRead, BitWrite, Error, Protocol, Settings};

use core::any::Any;
use std::convert::TryFrom;
use std::io;

/// The integer type that we will use to send length prefixes.
pub type SizeType = u32;

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

pub fn read_list<T>(
    read: &mut dyn BitRead,
    settings: &Settings,
    ctx: &mut dyn Any,
) -> Result<Vec<T>, Error>
where
    T: Protocol,
{
    let size = SizeType::read(read, settings, ctx)?;
    let size: usize = usize::try_from(size)?;

    read_items(size, read, settings, ctx).map(|i| i.collect())
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

/// Reads a length-prefixed list from a stream.
pub fn read_list_with_hints<T>(
    read: &mut dyn BitRead,
    settings: &Settings,
    ctx: &mut dyn Any,
    length: &FieldLength,
) -> Result<Vec<T>, Error>
where
    T: Protocol,
{
    let FieldLength { length, kind } = *length;
    match kind {
        LengthPrefixKind::Bytes => {
            let byte_count = length;

            // First, read all bytes of the list without processing them.
            let bytes: Vec<u8> = read_items(byte_count, read, settings, ctx)?.collect();
            let mut read_back_bytes = BitReader::endian(io::Cursor::new(bytes), BigEndian);

            // Then, parse the items until we reach the end of the buffer stream.
            let mut items = Vec::new();
            // FIXME: potential DoS vector, should timeout.
            while read_back_bytes.position_in_bits().unwrap() < (byte_count as u64) * 8 {
                let item = T::read(&mut read_back_bytes, settings, ctx)?;
                items.push(item);
            }

            Ok(items)
        }
        LengthPrefixKind::Elements => read_items(length, read, settings, ctx).map(|i| i.collect()),
    }
}

pub fn write_list_length_prefixed<'a, T, I>(
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

    let length = SizeType::try_from(elements.len())?;
    length.write(write, settings, ctx)?;
    write_items(elements.into_iter(), write, settings, ctx)?;

    Ok(())
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
