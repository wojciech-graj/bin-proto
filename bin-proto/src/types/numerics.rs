use crate::{BitField, BitRead, BitWrite, ByteOrder, Protocol, Result};

impl<Ctx> BitField<Ctx> for bool {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, bits: u32) -> Result<Self> {
        if read.read_u8_bf(bits)? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx, bits: u32) -> Result<()> {
        write.write_u8_bf(bits, if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl<Ctx> Protocol<Ctx> for bool {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        if read.read_u8()? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl<Ctx> Protocol<Ctx> for u8 {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        Ok(read.read_u8()?)
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        write.write_u8(*self)?;
        Ok(())
    }
}

impl<Ctx> Protocol<Ctx> for i8 {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self> {
        Ok(read.read_i8()?)
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<()> {
        write.write_i8(*self)?;
        Ok(())
    }
}

macro_rules! impl_for_numeric_cast {
    ($e:expr, $ty:ty, $cast_ty:ty) => {
        $e as $ty
    };
    ($e:expr,) => {
        $e
    };
}

macro_rules! impl_protocol_for_numeric {
    ($ty:ty => [$read_fn:ident : $write_fn:ident] $(as $cast_ty:ty)?) => {
        impl<Ctx> crate::Protocol<Ctx> for $ty {
            fn read(
                read: &mut dyn crate::BitRead,
                byte_order: crate::ByteOrder,
                _: &mut Ctx,
            ) -> crate::Result<Self> {
                Ok(impl_for_numeric_cast!(byte_order.$read_fn(read)?, $($ty, $cast_ty)?))
            }

            fn write(
                &self,
                write: &mut dyn crate::BitWrite,
                byte_order: crate::ByteOrder,
                _: &mut Ctx,
            ) -> crate::Result<()> {
                byte_order.$write_fn(*self $(as $cast_ty)?, write)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_bitfield_for_numeric {
    ($ty:ty => [$read_fn:ident : $write_fn:ident] $(as $cast_ty:ty)?) => {
        impl<Ctx> crate::BitField<Ctx> for $ty {
            fn read(
                read: &mut dyn crate::BitRead,
                _: crate::ByteOrder,
                _: &mut Ctx,
                bits: u32,
            ) -> crate::Result<Self> {
                Ok(impl_for_numeric_cast!(crate::BitRead::$read_fn(read, bits)?, $($ty, $cast_ty)?))
            }

            fn write(
                &self,
                write: &mut dyn crate::BitWrite,
                _: crate::ByteOrder,
                _: &mut Ctx,
                bits: u32,
            ) -> crate::Result<()> {
                crate::BitWrite::$write_fn(write, bits, *self $(as $cast_ty)?)?;
                Ok(())
            }
        }
    };
}

impl_protocol_for_numeric!(u16 => [read_u16 : write_u16]);
impl_protocol_for_numeric!(i16 => [read_i16 : write_i16]);
impl_protocol_for_numeric!(u32 => [read_u32 : write_u32]);
impl_protocol_for_numeric!(i32 => [read_i32 : write_i32]);
impl_protocol_for_numeric!(u64 => [read_u64 : write_u64]);
impl_protocol_for_numeric!(i64 => [read_i64 : write_i64]);
impl_protocol_for_numeric!(u128 => [read_u128 : write_u128]);
impl_protocol_for_numeric!(i128 => [read_i128 : write_i128]);
impl_protocol_for_numeric!(f32 => [read_f32 : write_f32]);
impl_protocol_for_numeric!(f64 => [read_f64 : write_f64]);

impl_bitfield_for_numeric!(u8 => [read_u8_bf : write_u8_bf]);
impl_bitfield_for_numeric!(i8 => [read_i8_bf : write_i8_bf]);
impl_bitfield_for_numeric!(u16 => [read_u16_bf : write_u16_bf]);
impl_bitfield_for_numeric!(i16 => [read_i16_bf : write_i16_bf]);
impl_bitfield_for_numeric!(u32 => [read_u32_bf : write_u32_bf]);
impl_bitfield_for_numeric!(i32 => [read_i32_bf : write_i32_bf]);

#[cfg(target_pointer_width = "16")]
mod size {
    impl_protocol_for_numeric!(usize => [read_u16 : write_u16] as u16);
    impl_bitfield_for_numeric!(usize => [read_u16_bf : write_u16_bf] as u16);
    impl_protocol_for_numeric!(isize => [read_i16 : write_i16] as i16);
    impl_bitfield_for_numeric!(isize => [read_i16_bf : write_i16_bf] as i16);
}

#[cfg(target_pointer_width = "32")]
mod size {
    impl_protocol_for_numeric!(usize => [read_u32 : write_u32] as u32);
    impl_bitfield_for_numeric!(usize => [read_u32_bf : write_u32_bf] as u32);
    impl_protocol_for_numeric!(isize => [read_i32 : write_i32] as i32);
    impl_bitfield_for_numeric!(isize => [read_i32_bf : write_i32_bf] as i32);
}

#[cfg(target_pointer_width = "64")]
mod size {
    impl_protocol_for_numeric!(usize => [read_u64 : write_u64] as u64);
    impl_bitfield_for_numeric!(usize => [read_u64_bf : write_u64_bf] as u64);
    impl_protocol_for_numeric!(isize => [read_i64 : write_i64] as i64);
    impl_bitfield_for_numeric!(isize => [read_i64_bf : write_i64_bf] as i64);
}
