use crate::{hint, BitField, BitRead, BitWrite, Error, Parcel, Settings};

use num_traits::{FromPrimitive, ToPrimitive};

/// An integer value that can be serialized and deserialized.
pub trait Integer: Parcel + FromPrimitive + ToPrimitive {}

impl BitField for bool {
    fn read_field(
        read: &mut dyn BitRead,
        bits: u32,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        if read.read_u8_bf(bits)? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        bits: u32,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        write.write_u8_bf(bits, if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Parcel for bool {
    fn read_field(
        read: &mut dyn BitRead,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        if read.read_u8()? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Parcel for u8 {
    fn read_field(
        read: &mut dyn BitRead,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        Ok(read.read_u8()?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        write.write_u8(*self)?;
        Ok(())
    }
}

impl Parcel for i8 {
    fn read_field(
        read: &mut dyn BitRead,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        Ok(read.read_i8()?)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        write.write_i8(*self)?;
        Ok(())
    }
}

macro_rules! impl_parcel_for_numeric {
    ($ty:ident => [$read_fn:ident : $write_fn:ident]) => {
        impl Parcel for $ty {
            fn read_field(
                read: &mut dyn BitRead,
                settings: &Settings,
                _: &mut hint::Hints,
            ) -> Result<Self, Error> {
                settings.byte_order.$read_fn(read)
            }

            fn write_field(
                &self,
                write: &mut dyn BitWrite,
                settings: &Settings,
                _: &mut hint::Hints,
            ) -> Result<(), Error> {
                settings.byte_order.$write_fn(*self, write)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_bitfield_for_numeric {
    ($ty:ident => [$read_fn:ident : $write_fn:ident]) => {
        impl BitField for $ty {
            fn read_field(
                read: &mut dyn BitRead,
                bits: u32,
                _: &Settings,
                _: &mut hint::Hints,
            ) -> Result<Self, Error> {
                Ok(BitRead::$read_fn(read, bits)?)
            }

            fn write_field(
                &self,
                write: &mut dyn BitWrite,
                bits: u32,
                _: &Settings,
                _: &mut hint::Hints,
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
impl_parcel_for_numeric!(f32 => [read_f32 : write_f32]);
impl_parcel_for_numeric!(f64 => [read_f64 : write_f64]);

impl_bitfield_for_numeric!(u8 => [read_u8_bf : write_u8_bf]);
impl_bitfield_for_numeric!(i8 => [read_i8_bf : write_i8_bf]);
impl_bitfield_for_numeric!(u16 => [read_u16_bf : write_u16_bf]);
impl_bitfield_for_numeric!(i16 => [read_i16_bf : write_i16_bf]);
impl_bitfield_for_numeric!(u32 => [read_u32_bf : write_u32_bf]);
impl_bitfield_for_numeric!(i32 => [read_i32_bf : write_i32_bf]);

impl Integer for u8 {}
impl Integer for i8 {}
impl Integer for u16 {}
impl Integer for i16 {}
impl Integer for u32 {}
impl Integer for i32 {}
impl Integer for u64 {}
impl Integer for i64 {}
