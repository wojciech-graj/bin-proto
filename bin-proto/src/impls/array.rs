use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{util, BitDecode, BitEncode, Result};
use core::mem::MaybeUninit;

impl<Ctx, T, const N: usize> BitDecode<Ctx> for [T; N]
where
    T: BitDecode<Ctx>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let elements = util::decode_items::<_, E, _, _>(N, read, ctx);
        let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
        let array_ptr = array.as_mut_ptr().cast::<T>();
        unsafe {
            for (i, item) in elements.enumerate() {
                array_ptr.add(i).write(item?);
            }
            Ok(array.assume_init())
        }
    }
}

impl<Ctx, T, const N: usize> BitEncode<Ctx> for [T; N]
where
    T: BitEncode<Ctx> + Sized,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.iter(), write, ctx)
    }
}

test_codec!([u8; 4]; [0, 1, 2, 3] => [0x00, 0x01, 0x02, 0x03]);
test_roundtrip!([u8; 4]);
