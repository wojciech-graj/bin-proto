//! Helper functions for dealing with sets or lists of parcels.

use bitstream_io::{BigEndian, BitReader};

use crate::hint::FieldLength;
use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

use std::convert::TryFrom;
use std::io;

/// The integer type that we will use to send length prefixes.
pub type SizeType = u32;

/// Reads a string of specified length from a stream.
pub fn read_string(
    byte_count: usize,
    read: &mut dyn BitRead,
    settings: &Settings,
) -> Result<String, Error> {
    let bytes: Vec<u8> = read_items(byte_count, read, settings)?.collect();
    String::from_utf8(bytes).map_err(Into::into)
}

/// Reads a specified number of items from a stream.
pub fn read_items<T>(
    item_count: usize,
    read: &mut dyn BitRead,
    settings: &Settings,
) -> Result<impl Iterator<Item = T>, Error>
where
    T: Parcel,
{
    let mut elements = Vec::with_capacity(item_count);

    for _ in 0..item_count {
        let element = T::read(read, settings)?;
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
) -> Result<(), Error>
where
    T: Parcel + 'a,
{
    for item in items.into_iter() {
        item.write(write, settings)?;
    }
    Ok(())
}

pub fn read_list_nohint<T>(read: &mut dyn BitRead, settings: &Settings) -> Result<Vec<T>, Error>
where
    T: Parcel,
{
    let size = SizeType::read(read, settings)?;
    let size: usize = usize::try_from(size)?;

    read_items(size, read, settings).map(|i| i.collect())
}

pub fn read_list_flexible<T>(read: &mut dyn BitRead, settings: &Settings) -> Result<Vec<T>, Error>
where
    T: Parcel,
{
    let mut items = Vec::new();
    loop {
        let item = match T::read(read, settings) {
            Ok(item) => item,
            Err(Error::IO(e)) => {
                return if e.kind() == io::ErrorKind::UnexpectedEof {
                    Ok(items)
                } else {
                    Err(e.into())
                }
            }
            Err(e) => return Err(e.into()),
        };
        items.push(item);
    }
}

/// Reads a length-prefixed list from a stream.
pub fn read_list<T>(
    read: &mut dyn BitRead,
    settings: &Settings,
    hints: &mut hint::Hints,
) -> Result<Vec<T>, Error>
where
    T: Parcel,
{
    match hints.current_field_length() {
        Some(FieldLength { length, kind }) => {
            match kind {
                hint::LengthPrefixKind::Bytes => {
                    let byte_count = length;

                    // First, read all bytes of the list without processing them.
                    let bytes: Vec<u8> = read_items(byte_count, read, settings)?.collect();
                    let mut read_back_bytes = BitReader::endian(io::Cursor::new(bytes), BigEndian);

                    // Then, parse the items until we reach the end of the buffer stream.
                    let mut items = Vec::new();
                    // FIXME: potential DoS vector, should timeout.
                    while read_back_bytes.position_in_bits().unwrap() < (byte_count as u64) * 8 {
                        let item = T::read(&mut read_back_bytes, settings)?;
                        items.push(item);
                    }

                    Ok(items)
                }
                hint::LengthPrefixKind::Elements => {
                    read_items(length, read, settings).map(|i| i.collect())
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn write_list_nohint<'a, T, I>(
    elements: I,
    write: &mut dyn BitWrite,
    settings: &Settings,
) -> Result<(), Error>
where
    T: Parcel + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let elements: Vec<_> = elements.into_iter().collect();

    let length = SizeType::try_from(elements.len())?;
    length.write(write, settings)?;
    write_items(elements.into_iter(), write, settings)?;

    Ok(())
}

/// BitWrites a length-prefixed list to a stream.
pub fn write_list<'a, T, I>(
    elements: I,
    write: &mut dyn BitWrite,
    settings: &Settings,
) -> Result<(), Error>
where
    T: Parcel + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let elements: Vec<_> = elements.into_iter().collect();

    write_items(elements.into_iter(), write, settings)?;

    Ok(())
}
