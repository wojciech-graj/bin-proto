use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

use bitstream_io::{BitRead, BitWrite, Endianness};

use crate::{BitDecode, BitEncode, Bits, Result};

impl<Ctx> BitDecode<Ctx, Bits> for bool {
    fn decode<R, E>(read: &mut R, _: &mut Ctx, Bits(bits): Bits) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        if read.read_var::<u8>(bits)? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl<Ctx> BitEncode<Ctx, Bits> for bool {
    fn encode<W, E>(&self, write: &mut W, _: &mut Ctx, Bits(bits): Bits) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        write.write_var(bits, u8::from(*self))?;
        Ok(())
    }
}

impl<Ctx> BitDecode<Ctx> for bool {
    fn decode<R, E>(read: &mut R, _: &mut Ctx, (): ()) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        if read.read_as_to::<E, u8>()? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl<Ctx> BitEncode<Ctx> for bool {
    fn encode<W, E>(&self, write: &mut W, _: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        write.write_as_from::<E, _>(u8::from(*self))?;
        Ok(())
    }
}

macro_rules! impl_codec_for_numeric_unordered {
    ($ty:ty => $data_ty:ty) => {
        impl<Ctx> $crate::BitDecode<Ctx> for $ty {
            fn decode<R, E>(read: &mut R, _: &mut Ctx, (): ()) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                Ok(::core::convert::TryInto::try_into(
                    ::bitstream_io::BitRead::read_as_to::<E, $data_ty>(read)?,
                )?)
            }
        }

        impl<Ctx> $crate::BitEncode<Ctx> for $ty {
            fn encode<W, E>(&self, write: &mut W, _: &mut Ctx, (): ()) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness,
            {
                ::bitstream_io::BitWrite::write_as_from::<E, $data_ty>(
                    write,
                    ::core::convert::TryInto::try_into(*self)?,
                )?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_codec_for_numeric {
    ($ty:ty $(: $thru:ident $fallible:tt)? => $data_ty:ty) => {
        impl<Ctx> $crate::BitDecode<Ctx> for $ty {
            fn decode<R, E>(
                read: &mut R,
                _: &mut Ctx,
                (): (),
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                Ok(::core::convert::TryInto::try_into(
                    $(::core::convert::TryInto::<$thru>::try_into)?(
                        ::bitstream_io::BitRead::read_as_to::<E, $data_ty>(read)?
                    )$($fallible)?,
                )?)
            }
        }

        impl<Ctx> $crate::BitEncode<Ctx> for $ty {
            fn encode<W, E>(
                &self,
                write: &mut W,
                _: &mut Ctx,
                (): (),
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness
            {
                ::bitstream_io::BitWrite::write_as_from::<E, $data_ty>(
                    write,
                    ::core::convert::TryInto::try_into(
                        $(::core::convert::TryInto::<$thru>::try_into)?(*self)$($fallible)?,
                    )?,
                )?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_bitfield_for_numeric {
    ($ty:ty $(: $thru:ident $fallible:tt)? => $data_ty:ty) => {
        impl<Ctx> $crate::BitDecode<Ctx, $crate::Bits> for $ty {
            fn decode<R, E>(
                read: &mut R,
                _: &mut Ctx,
                $crate::Bits(bits): $crate::Bits,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                Ok(::core::convert::TryInto::try_into(
                    $(::core::convert::TryInto::<$thru>::try_into)?(::bitstream_io::BitRead::read_var::<$data_ty>(
                        read, bits
                    )$($fallible)?)?,
                )?)
            }
        }

        impl<Ctx> $crate::BitEncode<Ctx, $crate::Bits> for $ty {
            fn encode<W, E>(
                &self,
                write: &mut W,
                _: &mut Ctx,
                $crate::Bits(bits): $crate::Bits,
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness
            {
                ::bitstream_io::BitWrite::write_var::<$data_ty>(
                    write,
                    bits,
                    ::core::convert::TryInto::try_into(
                        $(::core::convert::TryInto::<$thru>::try_into)?(*self)$($fallible)?,
                    )?,
                )?;
                Ok(())
            }
        }
    };
}

impl_codec_for_numeric_unordered!(u8 => u8);
impl_codec_for_numeric_unordered!(i8 => i8);

impl_codec_for_numeric_unordered!(NonZeroU8 => u8);
impl_codec_for_numeric_unordered!(NonZeroI8 => i8);

impl_codec_for_numeric!(u16 => u16);
impl_codec_for_numeric!(i16 => i16);
impl_codec_for_numeric!(u32 => u32);
impl_codec_for_numeric!(i32 => i32);
impl_codec_for_numeric!(u64 => u64);
impl_codec_for_numeric!(i64 => i64);
impl_codec_for_numeric!(u128 => u128);
impl_codec_for_numeric!(i128 => i128);

impl_codec_for_numeric!(NonZeroU16 => u16);
impl_codec_for_numeric!(NonZeroI16 => i16);
impl_codec_for_numeric!(NonZeroU32 => u32);
impl_codec_for_numeric!(NonZeroI32 => i32);
impl_codec_for_numeric!(NonZeroU64 => u64);
impl_codec_for_numeric!(NonZeroI64 => i64);
impl_codec_for_numeric!(NonZeroU128 => u128);
impl_codec_for_numeric!(NonZeroI128 => i128);

impl_codec_for_numeric!(f32 => f32);
impl_codec_for_numeric!(f64 => f64);

impl_bitfield_for_numeric!(u8 => u8);
impl_bitfield_for_numeric!(i8 => i8);
impl_bitfield_for_numeric!(u16 => u16);
impl_bitfield_for_numeric!(i16 => i16);
impl_bitfield_for_numeric!(u32 => u32);
impl_bitfield_for_numeric!(i32 => i32);
impl_bitfield_for_numeric!(u64 => u64);
impl_bitfield_for_numeric!(i64 => i64);

impl_bitfield_for_numeric!(NonZeroU8 => u8);
impl_bitfield_for_numeric!(NonZeroI8 => i8);
impl_bitfield_for_numeric!(NonZeroU16 => u16);
impl_bitfield_for_numeric!(NonZeroI16 => i16);
impl_bitfield_for_numeric!(NonZeroU32 => u32);
impl_bitfield_for_numeric!(NonZeroI32 => i32);

#[cfg(target_pointer_width = "16")]
mod size {
    use core::num::{NonZeroIsize, NonZeroUsize};

    impl_codec_for_numeric!(usize => u16);
    impl_bitfield_for_numeric!(usize => u16);

    impl_codec_for_numeric!(NonZeroUsize: usize? => u16);
    impl_bitfield_for_numeric!(NonZeroUsize: usize? => u16);

    impl_codec_for_numeric!(isize => i16);
    impl_bitfield_for_numeric!(isize => i16);

    impl_codec_for_numeric!(NonZeroIsize: isize? => i16);
    impl_bitfield_for_numeric!(NonZeroIsize: isize? => i16);
}

#[cfg(target_pointer_width = "32")]
mod size {
    use core::num::{NonZeroIsize, NonZeroUsize};

    impl_codec_for_numeric!(usize => u32);
    impl_bitfield_for_numeric!(usize => u32);

    impl_codec_for_numeric!(NonZeroUsize: usize? => u32);
    impl_bitfield_for_numeric!(NonZeroUsize: usize? => u32);

    impl_codec_for_numeric!(isize => i32);
    impl_bitfield_for_numeric!(isize => i32);

    impl_codec_for_numeric!(NonZeroIsize: isize? => i32);
    impl_bitfield_for_numeric!(NonZeroIsize: isize? => i32);
}

#[cfg(target_pointer_width = "64")]
mod size {
    use core::num::{NonZeroIsize, NonZeroUsize};

    impl_codec_for_numeric!(usize => u64);
    impl_bitfield_for_numeric!(usize => u64);

    impl_codec_for_numeric!(NonZeroUsize: usize? => u64);
    impl_bitfield_for_numeric!(NonZeroUsize: usize? => u64);

    impl_codec_for_numeric!(isize => i64);
    impl_bitfield_for_numeric!(isize => i64);

    impl_codec_for_numeric!(NonZeroIsize: isize? => i64);
    impl_bitfield_for_numeric!(NonZeroIsize: isize? => i64);
}
