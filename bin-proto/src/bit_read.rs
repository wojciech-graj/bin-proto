#[cfg(feature = "std")]
use std::io;

use bitstream_io::{BE, LE};
#[cfg(not(feature = "std"))]
use core2::io;

/// A bit-level equivalent of `std::io::Read`. An object-safe wrapper over
/// `bitstream_io::BitRead`.
pub trait BitRead {
    fn read_bit(&mut self) -> io::Result<bool>;
    fn skip(&mut self, bits: u32) -> io::Result<()>;
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()>;
    fn byte_aligned(&self) -> bool;
    fn byte_align(&mut self);

    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_i8(&mut self) -> io::Result<i8>;

    fn read_u16_le(&mut self) -> io::Result<u16>;
    fn read_u16_be(&mut self) -> io::Result<u16>;
    fn read_i16_le(&mut self) -> io::Result<i16>;
    fn read_i16_be(&mut self) -> io::Result<i16>;
    fn read_u32_le(&mut self) -> io::Result<u32>;
    fn read_u32_be(&mut self) -> io::Result<u32>;
    fn read_i32_le(&mut self) -> io::Result<i32>;
    fn read_i32_be(&mut self) -> io::Result<i32>;
    fn read_u64_le(&mut self) -> io::Result<u64>;
    fn read_u64_be(&mut self) -> io::Result<u64>;
    fn read_i64_le(&mut self) -> io::Result<i64>;
    fn read_i64_be(&mut self) -> io::Result<i64>;
    fn read_u128_le(&mut self) -> io::Result<u128>;
    fn read_u128_be(&mut self) -> io::Result<u128>;
    fn read_i128_le(&mut self) -> io::Result<i128>;
    fn read_i128_be(&mut self) -> io::Result<i128>;
    fn read_f32_le(&mut self) -> io::Result<f32>;
    fn read_f32_be(&mut self) -> io::Result<f32>;
    fn read_f64_le(&mut self) -> io::Result<f64>;
    fn read_f64_be(&mut self) -> io::Result<f64>;

    fn read_u8_bf(&mut self, bits: u32) -> io::Result<u8>;
    fn read_i8_bf(&mut self, bits: u32) -> io::Result<i8>;
    fn read_u16_bf(&mut self, bits: u32) -> io::Result<u16>;
    fn read_i16_bf(&mut self, bits: u32) -> io::Result<i16>;
    fn read_u32_bf(&mut self, bits: u32) -> io::Result<u32>;
    fn read_i32_bf(&mut self, bits: u32) -> io::Result<i32>;
    fn read_u64_bf(&mut self, bits: u32) -> io::Result<u64>;
    fn read_i64_bf(&mut self, bits: u32) -> io::Result<i64>;
}

impl<T> BitRead for T
where
    T: bitstream_io::BitRead,
{
    fn read_bit(&mut self) -> io::Result<bool> {
        bitstream_io::BitRead::read_bit(self)
    }

    fn skip(&mut self, bits: u32) -> io::Result<()> {
        bitstream_io::BitRead::skip(self, bits)
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        bitstream_io::BitRead::read_bytes(self, buf)
    }

    fn byte_aligned(&self) -> bool {
        bitstream_io::BitRead::byte_aligned(self)
    }

    fn byte_align(&mut self) {
        bitstream_io::BitRead::byte_align(self);
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        bitstream_io::BitRead::read_to::<u8>(self)
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        bitstream_io::BitRead::read_to::<i8>(self)
    }

    fn read_u16_le(&mut self) -> io::Result<u16> {
        bitstream_io::BitRead::read_as_to::<LE, u16>(self)
    }

    fn read_u16_be(&mut self) -> io::Result<u16> {
        bitstream_io::BitRead::read_as_to::<BE, u16>(self)
    }

    fn read_i16_le(&mut self) -> io::Result<i16> {
        bitstream_io::BitRead::read_as_to::<LE, i16>(self)
    }

    fn read_i16_be(&mut self) -> io::Result<i16> {
        bitstream_io::BitRead::read_as_to::<BE, i16>(self)
    }

    fn read_u32_le(&mut self) -> io::Result<u32> {
        bitstream_io::BitRead::read_as_to::<LE, u32>(self)
    }

    fn read_u32_be(&mut self) -> io::Result<u32> {
        bitstream_io::BitRead::read_as_to::<BE, u32>(self)
    }

    fn read_i32_le(&mut self) -> io::Result<i32> {
        bitstream_io::BitRead::read_as_to::<LE, i32>(self)
    }

    fn read_i32_be(&mut self) -> io::Result<i32> {
        bitstream_io::BitRead::read_as_to::<BE, i32>(self)
    }

    fn read_u64_le(&mut self) -> io::Result<u64> {
        bitstream_io::BitRead::read_as_to::<LE, u64>(self)
    }

    fn read_u64_be(&mut self) -> io::Result<u64> {
        bitstream_io::BitRead::read_as_to::<BE, u64>(self)
    }

    fn read_i64_le(&mut self) -> io::Result<i64> {
        bitstream_io::BitRead::read_as_to::<LE, i64>(self)
    }

    fn read_i64_be(&mut self) -> io::Result<i64> {
        bitstream_io::BitRead::read_as_to::<BE, i64>(self)
    }

    fn read_u128_le(&mut self) -> io::Result<u128> {
        bitstream_io::BitRead::read_as_to::<LE, u128>(self)
    }

    fn read_u128_be(&mut self) -> io::Result<u128> {
        bitstream_io::BitRead::read_as_to::<BE, u128>(self)
    }

    fn read_i128_le(&mut self) -> io::Result<i128> {
        bitstream_io::BitRead::read_as_to::<LE, i128>(self)
    }

    fn read_i128_be(&mut self) -> io::Result<i128> {
        bitstream_io::BitRead::read_as_to::<BE, i128>(self)
    }

    fn read_f32_le(&mut self) -> io::Result<f32> {
        bitstream_io::BitRead::read_as_to::<LE, f32>(self)
    }

    fn read_f32_be(&mut self) -> io::Result<f32> {
        bitstream_io::BitRead::read_as_to::<BE, f32>(self)
    }

    fn read_f64_le(&mut self) -> io::Result<f64> {
        bitstream_io::BitRead::read_as_to::<LE, f64>(self)
    }

    fn read_f64_be(&mut self) -> io::Result<f64> {
        bitstream_io::BitRead::read_as_to::<BE, f64>(self)
    }

    fn read_u8_bf(&mut self, bits: u32) -> io::Result<u8> {
        bitstream_io::BitRead::read(self, bits)
    }

    fn read_i8_bf(&mut self, bits: u32) -> io::Result<i8> {
        bitstream_io::BitRead::read_signed(self, bits)
    }

    fn read_u16_bf(&mut self, bits: u32) -> io::Result<u16> {
        bitstream_io::BitRead::read(self, bits)
    }

    fn read_i16_bf(&mut self, bits: u32) -> io::Result<i16> {
        bitstream_io::BitRead::read_signed(self, bits)
    }

    fn read_u32_bf(&mut self, bits: u32) -> io::Result<u32> {
        bitstream_io::BitRead::read(self, bits)
    }

    fn read_i32_bf(&mut self, bits: u32) -> io::Result<i32> {
        bitstream_io::BitRead::read_signed(self, bits)
    }

    fn read_u64_bf(&mut self, bits: u32) -> io::Result<u64> {
        bitstream_io::BitRead::read(self, bits)
    }

    fn read_i64_bf(&mut self, bits: u32) -> io::Result<i64> {
        bitstream_io::BitRead::read_signed(self, bits)
    }
}
