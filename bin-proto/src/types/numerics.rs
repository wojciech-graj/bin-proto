use crate::{BitField, BitRead, BitWrite, ByteOrder, Error, Protocol};

impl<Ctx> BitField<Ctx> for bool {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx, bits: u32) -> Result<Self, Error> {
        if read.read_u8_bf(bits)? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        _: ByteOrder,
        _: &mut Ctx,
        bits: u32,
    ) -> Result<(), Error> {
        write.write_u8_bf(bits, if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl<Ctx> Protocol<Ctx> for bool {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self, Error> {
        if read.read_u8()? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<(), Error> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl<Ctx> Protocol<Ctx> for u8 {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self, Error> {
        Ok(read.read_u8()?)
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<(), Error> {
        write.write_u8(*self)?;
        Ok(())
    }
}

impl<Ctx> Protocol<Ctx> for i8 {
    fn read(read: &mut dyn BitRead, _: ByteOrder, _: &mut Ctx) -> Result<Self, Error> {
        Ok(read.read_i8()?)
    }

    fn write(&self, write: &mut dyn BitWrite, _: ByteOrder, _: &mut Ctx) -> Result<(), Error> {
        write.write_i8(*self)?;
        Ok(())
    }
}

macro_rules! impl_parcel_for_numeric {
    ($ty:ident => [$read_fn:ident : $write_fn:ident]) => {
        impl<Ctx> Protocol<Ctx> for $ty {
            fn read(
                read: &mut dyn BitRead,
                byte_order: ByteOrder,
                _: &mut Ctx,
            ) -> Result<Self, Error> {
                byte_order.$read_fn(read)
            }

            fn write(
                &self,
                write: &mut dyn BitWrite,
                byte_order: ByteOrder,
                _: &mut Ctx,
            ) -> Result<(), Error> {
                byte_order.$write_fn(*self, write)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_bitfield_for_numeric {
    ($ty:ident => [$read_fn:ident : $write_fn:ident]) => {
        impl<Ctx> BitField<Ctx> for $ty {
            fn read(
                read: &mut dyn BitRead,
                _: ByteOrder,
                _: &mut Ctx,
                bits: u32,
            ) -> Result<Self, Error> {
                Ok(BitRead::$read_fn(read, bits)?)
            }

            fn write(
                &self,
                write: &mut dyn BitWrite,
                _: ByteOrder,
                _: &mut Ctx,
                bits: u32,
            ) -> Result<(), Error> {
                BitWrite::$write_fn(write, bits, *self)?;
                Ok(())
            }
        }
    };
}

impl_parcel_for_numeric!(u16 => [read_u16 : write_u16]);
impl_parcel_for_numeric!(i16 => [read_i16 : write_i16]);
impl_parcel_for_numeric!(u32 => [read_u32 : write_u32]);
impl_parcel_for_numeric!(i32 => [read_i32 : write_i32]);
impl_parcel_for_numeric!(u64 => [read_u64 : write_u64]);
impl_parcel_for_numeric!(i64 => [read_i64 : write_i64]);
impl_parcel_for_numeric!(u128 => [read_u128 : write_u128]);
impl_parcel_for_numeric!(i128 => [read_i128 : write_i128]);
impl_parcel_for_numeric!(f32 => [read_f32 : write_f32]);
impl_parcel_for_numeric!(f64 => [read_f64 : write_f64]);

impl_bitfield_for_numeric!(u8 => [read_u8_bf : write_u8_bf]);
impl_bitfield_for_numeric!(i8 => [read_i8_bf : write_i8_bf]);
impl_bitfield_for_numeric!(u16 => [read_u16_bf : write_u16_bf]);
impl_bitfield_for_numeric!(i16 => [read_i16_bf : write_i16_bf]);
impl_bitfield_for_numeric!(u32 => [read_u32_bf : write_u32_bf]);
impl_bitfield_for_numeric!(i32 => [read_i32_bf : write_i32_bf]);
