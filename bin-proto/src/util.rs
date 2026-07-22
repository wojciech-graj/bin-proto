//! Helper functions for dealing with iterators

use crate::{BitDecode, BitEncode, Error, Result};

use bitstream_io::{BitRead, BitWrite, Endianness};
use core::iter;
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use no_std_io2::io;

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

/// Upper bound, in bytes, on capacity reserved up front for a length-prefixed
/// collection, so a hostile length prefix can't trigger a huge allocation (or a
/// `capacity overflow` panic) before any element is read.
#[cfg(feature = "alloc")]
const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

/// Capacity to reserve for `len` upcoming `T`s, capped by [`MAX_PREALLOC_BYTES`].
///
/// `len` comes from an untrusted length prefix and is only a hint: the collection
/// still grows on push, so capping never changes what decodes, it only bounds the
/// eager reservation.
#[cfg(feature = "alloc")]
#[inline]
pub(crate) fn cautious_capacity<T>(len: usize) -> usize {
    let elem = core::mem::size_of::<T>().max(1);
    len.min(MAX_PREALLOC_BYTES / elem)
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::cautious_capacity;

    #[test]
    fn caps_hostile_count() {
        assert_eq!(cautious_capacity::<u8>(usize::MAX), 1024 * 1024);
        assert_eq!(cautious_capacity::<u128>(usize::MAX), 1024 * 1024 / 16);
    }

    #[test]
    fn passes_through_legitimate_count() {
        assert_eq!(cautious_capacity::<u32>(10), 10);
        assert_eq!(cautious_capacity::<u8>(0), 0);
    }

    #[test]
    fn zero_sized_type_does_not_divide_by_zero() {
        assert_eq!(cautious_capacity::<()>(5), 5);
    }
}
