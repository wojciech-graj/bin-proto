use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

use crate::{
    BitFieldRead, BitFieldWrite, BitRead, BitWrite, ByteOrder, ProtocolRead, ProtocolWrite, Result,
};

impl<Ctx> BitFieldRead<Ctx> for bool {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, bits: u32) -> Result<Self> {
        if read.read_u8_bf(bits)? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl<Ctx> BitFieldWrite<Ctx> for bool {
    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx, bits: u32) -> Result<()> {
        write.write_u8_bf(bits, (*self).into())?;
        Ok(())
    }
}

impl<Ctx> ProtocolRead<Ctx> for bool {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        if read.read_u8()? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl<Ctx> ProtocolWrite<Ctx> for bool {
    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        write.write_u8((*self).into())?;
        Ok(())
    }
}

macro_rules! impl_protocol_for_numeric_unordered {
    ($ty:ty => [$read_fn:ident : $write_fn:ident]) => {
        impl<Ctx> $crate::ProtocolRead<Ctx> for $ty {
            fn read(
                read: &mut dyn $crate::BitRead,
                _: $crate::ByteOrder,
                _: &mut Ctx,
            ) -> $crate::Result<Self> {
                Ok(::core::convert::TryInto::try_into(read.$read_fn()?)?)
            }
        }

        impl<Ctx> $crate::ProtocolWrite<Ctx> for $ty {
            fn write(
                &self,
                write: &mut dyn $crate::BitWrite,
                _: $crate::ByteOrder,
                _: &mut Ctx,
            ) -> $crate::Result<()> {
                write.$write_fn(::core::convert::TryInto::try_into(*self)?)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_protocol_for_numeric {
    ($ty:ty $(: $thru:ident $fallible:tt)? => [$read_fn:ident : $write_fn:ident]) => {
        impl<Ctx> $crate::ProtocolRead<Ctx> for $ty {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                _: &mut Ctx,
            ) -> $crate::Result<Self> {
                Ok(::core::convert::TryInto::try_into(
                    $(::core::convert::TryInto::<$thru>::try_into)?(
                        byte_order.$read_fn(read)?
                    )$($fallible)?,
                )?)
            }
        }

        impl<Ctx> $crate::ProtocolWrite<Ctx> for $ty {
            fn write(
                &self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                _: &mut Ctx,
            ) -> $crate::Result<()> {
                byte_order.$write_fn(
                    ::core::convert::TryInto::try_into(
                        $(::core::convert::TryInto::<$thru>::try_into)?(*self)$($fallible)?,
                    )?,
                    write,
                )?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_bitfield_for_numeric {
    ($ty:ty $(: $thru:ident $fallible:tt)? => [$read_fn:ident : $write_fn:ident]) => {
        impl<Ctx> $crate::BitFieldRead<Ctx> for $ty {
            fn read(
                read: &mut dyn $crate::BitRead,
                _: $crate::ByteOrder,
                _: &mut Ctx,
                bits: u32,
            ) -> $crate::Result<Self> {
                Ok(::core::convert::TryInto::try_into(
                    $(::core::convert::TryInto::<$thru>::try_into)?($crate::BitRead::$read_fn(
                        read, bits,
                    )$($fallible)?)?,
                )?)
            }
        }

        impl<Ctx> $crate::BitFieldWrite<Ctx> for $ty {
            fn write(
                &self,
                write: &mut dyn $crate::BitWrite,
                _: $crate::ByteOrder,
                _: &mut Ctx,
                bits: u32,
            ) -> $crate::Result<()> {
                $crate::BitWrite::$write_fn(
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

impl_protocol_for_numeric_unordered!(u8 => [read_u8 : write_u8]);
impl_protocol_for_numeric_unordered!(i8 => [read_i8 : write_i8]);

impl_protocol_for_numeric_unordered!(NonZeroU8 => [read_u8 : write_u8]);
impl_protocol_for_numeric_unordered!(NonZeroI8 => [read_i8 : write_i8]);

impl_protocol_for_numeric!(u16 => [read_u16 : write_u16]);
impl_protocol_for_numeric!(i16 => [read_i16 : write_i16]);
impl_protocol_for_numeric!(u32 => [read_u32 : write_u32]);
impl_protocol_for_numeric!(i32 => [read_i32 : write_i32]);
impl_protocol_for_numeric!(u64 => [read_u64 : write_u64]);
impl_protocol_for_numeric!(i64 => [read_i64 : write_i64]);
impl_protocol_for_numeric!(u128 => [read_u128 : write_u128]);
impl_protocol_for_numeric!(i128 => [read_i128 : write_i128]);

impl_protocol_for_numeric!(NonZeroU16 => [read_u16 : write_u16]);
impl_protocol_for_numeric!(NonZeroI16 => [read_i16 : write_i16]);
impl_protocol_for_numeric!(NonZeroU32 => [read_u32 : write_u32]);
impl_protocol_for_numeric!(NonZeroI32 => [read_i32 : write_i32]);
impl_protocol_for_numeric!(NonZeroU64 => [read_u64 : write_u64]);
impl_protocol_for_numeric!(NonZeroI64 => [read_i64 : write_i64]);
impl_protocol_for_numeric!(NonZeroU128 => [read_u128 : write_u128]);
impl_protocol_for_numeric!(NonZeroI128 => [read_i128 : write_i128]);

impl_protocol_for_numeric!(f32 => [read_f32 : write_f32]);
impl_protocol_for_numeric!(f64 => [read_f64 : write_f64]);

impl_bitfield_for_numeric!(u8 => [read_u8_bf : write_u8_bf]);
impl_bitfield_for_numeric!(i8 => [read_i8_bf : write_i8_bf]);
impl_bitfield_for_numeric!(u16 => [read_u16_bf : write_u16_bf]);
impl_bitfield_for_numeric!(i16 => [read_i16_bf : write_i16_bf]);
impl_bitfield_for_numeric!(u32 => [read_u32_bf : write_u32_bf]);
impl_bitfield_for_numeric!(i32 => [read_i32_bf : write_i32_bf]);
impl_bitfield_for_numeric!(u64 => [read_u64_bf : write_u64_bf]);
impl_bitfield_for_numeric!(i64 => [read_i64_bf : write_i64_bf]);

impl_bitfield_for_numeric!(NonZeroU8 => [read_u8_bf : write_u8_bf]);
impl_bitfield_for_numeric!(NonZeroI8 => [read_i8_bf : write_i8_bf]);
impl_bitfield_for_numeric!(NonZeroU16 => [read_u16_bf : write_u16_bf]);
impl_bitfield_for_numeric!(NonZeroI16 => [read_i16_bf : write_i16_bf]);
impl_bitfield_for_numeric!(NonZeroU32 => [read_u32_bf : write_u32_bf]);
impl_bitfield_for_numeric!(NonZeroI32 => [read_i32_bf : write_i32_bf]);

#[cfg(target_pointer_width = "16")]
mod size {
    use core::num::{NonZeroIsize, NonZeroUsize};

    impl_protocol_for_numeric!(usize => [read_u16 : write_u16]);
    impl_bitfield_for_numeric!(usize => [read_u16_bf : write_u16_bf]);

    impl_protocol_for_numeric!(NonZeroUsize: usize? => [read_u16 : write_u16]);
    impl_bitfield_for_numeric!(NonZeroUsize: usize? => [read_u16_bf : write_u16_bf]);

    impl_protocol_for_numeric!(isize => [read_i16 : write_i16]);
    impl_bitfield_for_numeric!(isize => [read_i16_bf : write_i16_bf]);

    impl_protocol_for_numeric!(NonZeroIsize: isize? => [read_i16 : write_i16]);
    impl_bitfield_for_numeric!(NonZeroIsize: isize? => [read_i16_bf : write_i16_bf]);
}

#[cfg(target_pointer_width = "32")]
mod size {
    use core::num::{NonZeroIsize, NonZeroUsize};

    impl_protocol_for_numeric!(usize => [read_u32 : write_u32]);
    impl_bitfield_for_numeric!(usize => [read_u32_bf : write_u32_bf]);

    impl_protocol_for_numeric!(NonZeroUsize: usize? => [read_u32 : write_u32]);
    impl_bitfield_for_numeric!(NonZeroUsize: usize? => [read_u32_bf : write_u32_bf]);

    impl_protocol_for_numeric!(isize => [read_i32 : write_i32]);
    impl_bitfield_for_numeric!(isize => [read_i32_bf : write_i32_bf]);

    impl_protocol_for_numeric!(NonZeroIsize: isize? => [read_i32 : write_i32]);
    impl_bitfield_for_numeric!(NonZeroIsize: isize? => [read_i32_bf : write_i32_bf]);
}

#[cfg(target_pointer_width = "64")]
mod size {
    use core::num::{NonZeroIsize, NonZeroUsize};

    impl_protocol_for_numeric!(usize => [read_u64 : write_u64]);
    impl_bitfield_for_numeric!(usize => [read_u64_bf : write_u64_bf]);

    impl_protocol_for_numeric!(NonZeroUsize: usize? => [read_u64 : write_u64]);
    impl_bitfield_for_numeric!(NonZeroUsize: usize? => [read_u64_bf : write_u64_bf]);

    impl_protocol_for_numeric!(isize => [read_i64 : write_i64]);
    impl_bitfield_for_numeric!(isize => [read_i64_bf : write_i64_bf]);

    impl_protocol_for_numeric!(NonZeroIsize: isize? => [read_i64 : write_i64]);
    impl_bitfield_for_numeric!(NonZeroIsize: isize? => [read_i64_bf : write_i64_bf]);
}
