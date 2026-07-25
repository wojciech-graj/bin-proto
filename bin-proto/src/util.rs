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

/// Reserve capacity for `len` upcoming elements without panicking.
///
/// Inherent `try_reserve` returns `Err` rather than aborting on capacity overflow or a
/// legitimately-too-large reservation, so a hostile length prefix degrades to a normal
/// [`Error::Alloc`] instead of an OOM/panic. `LinkedList`, `BTreeSet` and `BTreeMap`
/// have no contiguous backing buffer — they never pre-allocate on decode, so their
/// impl is a no-op.
#[cfg(feature = "alloc")]
pub trait Reservable<T> {
    /// Try to reserve capacity for `len` more elements, returning `Err` on failure.
    fn try_reserve_hint(&mut self, len: usize) -> Result<()>;
}

#[cfg(feature = "alloc")]
impl<T> Reservable<T> for alloc::vec::Vec<T> {
    #[inline]
    fn try_reserve_hint(&mut self, len: usize) -> Result<()> {
        Self::try_reserve(self, len).map_err(Error::from)
    }
}

#[cfg(feature = "alloc")]
impl<T> Reservable<T> for alloc::collections::VecDeque<T> {
    #[inline]
    fn try_reserve_hint(&mut self, len: usize) -> Result<()> {
        Self::try_reserve(self, len).map_err(Error::from)
    }
}

#[cfg(feature = "alloc")]
impl<T> Reservable<T> for alloc::collections::BinaryHeap<T> {
    #[inline]
    fn try_reserve_hint(&mut self, len: usize) -> Result<()> {
        Self::try_reserve(self, len).map_err(Error::from)
    }
}

#[cfg(feature = "std")]
impl<T, H> Reservable<T> for std::collections::HashSet<T, H>
where
    T: core::hash::Hash + core::cmp::Eq,
    H: core::hash::BuildHasher,
{
    #[inline]
    fn try_reserve_hint(&mut self, len: usize) -> Result<()> {
        Self::try_reserve(self, len).map_err(Error::from)
    }
}

#[cfg(feature = "std")]
impl<K, V, H> Reservable<(K, V)> for std::collections::HashMap<K, V, H>
where
    K: core::hash::Hash + core::cmp::Eq,
    H: core::hash::BuildHasher,
{
    #[inline]
    fn try_reserve_hint(&mut self, len: usize) -> Result<()> {
        Self::try_reserve(self, len).map_err(Error::from)
    }
}

// `LinkedList`, `BTreeSet` and `BTreeMap` have no contiguous backing buffer, so they
// never over-reserve on decode — the reservation is a no-op.
#[cfg(feature = "alloc")]
impl<T> Reservable<T> for alloc::collections::LinkedList<T> {
    #[inline]
    fn try_reserve_hint(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<T> Reservable<T> for alloc::collections::BTreeSet<T> {
    #[inline]
    fn try_reserve_hint(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<K, V> Reservable<(K, V)> for alloc::collections::BTreeMap<K, V> {
    #[inline]
    fn try_reserve_hint(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }
}
