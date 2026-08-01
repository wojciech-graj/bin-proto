//! Helper functions for dealing with iterators

use bitstream_io::{BitRead, BitWrite, Endianness};
use core::iter;

use crate::{error::ErrorKind, io, BitDecode, BitEncode, Result};

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

/// [`BitDecode`]s items until EOF.
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
        Err(e) if e.kind() == ErrorKind::Io(io::ErrorKind::UnexpectedEof) => None,
        other => Some(other),
    })
}

#[cfg(test)]
mod tests {
    use bitstream_io::{BigEndian, BitReader};

    use crate::Error;

    use super::*;

    #[derive(Debug)]
    struct CannotDecode;

    impl<Ctx> BitDecode<Ctx> for CannotDecode {
        fn decode<R, E>(_: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            Err(Error::msg(""))
        }
    }

    #[test]
    fn decode_items_to_eof_other_error() {
        assert_eq!(
            ErrorKind::Other,
            decode_items_to_eof::<_, BigEndian, _, CannotDecode>(
                &mut BitReader::<_, BigEndian>::new(&[] as &[u8]),
                &mut ()
            )
            .next()
            .unwrap()
            .unwrap_err()
            .kind()
        );
    }
}
