use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{util, BitDecode, BitEncode, Result};
use core::mem::MaybeUninit;

struct PartialGuard<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for PartialGuard<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(self.ptr, self.len));
        }
    }
}

impl<E, Ctx, T, const N: usize> BitDecode<E, Ctx> for [T; N]
where
    E: Endianness,
    T: BitDecode<E, Ctx>,
{
    fn decode<R>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead + ?Sized,
    {
        let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
        let mut guard = PartialGuard {
            ptr: array.as_mut_ptr().cast::<T>(),
            len: 0,
        };
        while guard.len < N {
            let item = T::decode(read, ctx, ())?;
            unsafe {
                guard.ptr.add(guard.len).write(item);
            }
            guard.len += 1;
        }
        core::mem::forget(guard);
        Ok(unsafe { array.assume_init() })
    }
}

impl<E, Ctx, T, const N: usize> BitEncode<E, Ctx> for [T; N]
where
    E: Endianness,
    T: BitEncode<E, Ctx>,
{
    fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite + ?Sized,
    {
        util::encode_items::<_, _, E, _, _>(self.iter(), write, ctx)
    }
}

test_codec!([u8; 4]; [0, 1, 2, 3] => [0x00, 0x01, 0x02, 0x03]);
test_roundtrip!([u8; 4]);

#[cfg(test)]
mod test {
    use core::cell::RefCell;

    use bitstream_io::BigEndian;

    use crate::{error::ErrorCause, BitDecodeExt, Error};

    use super::*;

    #[derive(Default)]
    struct MustDropState {
        decoded: bool,
        dropped: bool,
    }

    struct Ctx<'a>(&'a RefCell<MustDropState>);

    struct MustDrop<'a>(&'a RefCell<MustDropState>);

    impl<'a> Drop for MustDrop<'a> {
        fn drop(&mut self) {
            self.0.borrow_mut().dropped = true;
        }
    }

    impl<'a, E> BitDecode<E, Ctx<'a>> for MustDrop<'a>
    where
        E: Endianness,
    {
        fn decode<R>(_: &mut R, ctx: &mut Ctx<'a>, (): ()) -> Result<Self>
        where
            R: BitRead + ?Sized,
        {
            let mut state = ctx.0.borrow_mut();
            if state.decoded {
                Err(Error::from_inner(ErrorCause::Other("")))
            } else {
                state.decoded = true;
                Ok(Self(ctx.0))
            }
        }
    }

    #[test]
    fn partial_result_dropped() {
        let state = RefCell::new(MustDropState::default());
        assert!(
            <[MustDrop; 2]>::decode_bytes_ctx::<BigEndian, _, _>(&[], &mut Ctx(&state), ())
                .is_err()
        );
        assert!(state.borrow().dropped);
    }
}
