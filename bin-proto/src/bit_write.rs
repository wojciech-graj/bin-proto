#[cfg(feature = "std")]
use std::io;

use bitstream_io::{BE, LE};
#[cfg(not(feature = "std"))]
use core2::io;

/// A bit-level equivalent of [`Write`](std::io::Write). An object-safe wrapper over
/// [`bitstream_io::BitWrite`].
pub trait BitWrite {
    fn write_bit(&mut self, bit: bool) -> io::Result<()>;
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()>;
    fn byte_aligned(&self) -> bool;
    fn byte_align(&mut self) -> io::Result<()>;

    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    fn write_i8(&mut self, value: i8) -> io::Result<()>;

    fn write_u16_le(&mut self, value: u16) -> io::Result<()>;
    fn write_u16_be(&mut self, value: u16) -> io::Result<()>;
    fn write_i16_le(&mut self, value: i16) -> io::Result<()>;
    fn write_i16_be(&mut self, value: i16) -> io::Result<()>;
    fn write_u32_le(&mut self, value: u32) -> io::Result<()>;
    fn write_u32_be(&mut self, value: u32) -> io::Result<()>;
    fn write_i32_le(&mut self, value: i32) -> io::Result<()>;
    fn write_i32_be(&mut self, value: i32) -> io::Result<()>;
    fn write_u64_le(&mut self, value: u64) -> io::Result<()>;
    fn write_u64_be(&mut self, value: u64) -> io::Result<()>;
    fn write_i64_le(&mut self, value: i64) -> io::Result<()>;
    fn write_i64_be(&mut self, value: i64) -> io::Result<()>;
    fn write_u128_le(&mut self, value: u128) -> io::Result<()>;
    fn write_u128_be(&mut self, value: u128) -> io::Result<()>;
    fn write_i128_le(&mut self, value: i128) -> io::Result<()>;
    fn write_i128_be(&mut self, value: i128) -> io::Result<()>;
    fn write_f32_le(&mut self, value: f32) -> io::Result<()>;
    fn write_f32_be(&mut self, value: f32) -> io::Result<()>;
    fn write_f64_le(&mut self, value: f64) -> io::Result<()>;
    fn write_f64_be(&mut self, value: f64) -> io::Result<()>;

    fn write_u8_bf(&mut self, bits: u32, value: u8) -> io::Result<()>;
    fn write_i8_bf(&mut self, bits: u32, value: i8) -> io::Result<()>;
    fn write_u16_bf(&mut self, bits: u32, value: u16) -> io::Result<()>;
    fn write_i16_bf(&mut self, bits: u32, value: i16) -> io::Result<()>;
    fn write_u32_bf(&mut self, bits: u32, value: u32) -> io::Result<()>;
    fn write_i32_bf(&mut self, bits: u32, value: i32) -> io::Result<()>;
    fn write_u64_bf(&mut self, bits: u32, value: u64) -> io::Result<()>;
    fn write_i64_bf(&mut self, bits: u32, value: i64) -> io::Result<()>;
}

impl<T> BitWrite for T
where
    T: bitstream_io::BitWrite,
{
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        bitstream_io::BitWrite::write_bit(self, bit)
    }

    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        bitstream_io::BitWrite::write_bytes(self, buf)
    }

    fn byte_aligned(&self) -> bool {
        bitstream_io::BitWrite::byte_aligned(self)
    }

    fn byte_align(&mut self) -> io::Result<()> {
        bitstream_io::BitWrite::byte_align(self)
    }

    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        bitstream_io::BitWrite::write_from(self, value)
    }

    fn write_i8(&mut self, value: i8) -> io::Result<()> {
        bitstream_io::BitWrite::write_from(self, value)
    }

    fn write_u16_le(&mut self, value: u16) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_u16_be(&mut self, value: u16) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_i16_le(&mut self, value: i16) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_i16_be(&mut self, value: i16) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_u32_le(&mut self, value: u32) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_u32_be(&mut self, value: u32) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_i32_le(&mut self, value: i32) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_i32_be(&mut self, value: i32) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_u64_le(&mut self, value: u64) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_u64_be(&mut self, value: u64) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_i64_le(&mut self, value: i64) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_i64_be(&mut self, value: i64) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_u128_le(&mut self, value: u128) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_u128_be(&mut self, value: u128) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_i128_le(&mut self, value: i128) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_i128_be(&mut self, value: i128) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_f32_le(&mut self, value: f32) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_f32_be(&mut self, value: f32) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_f64_le(&mut self, value: f64) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<LE, _>(self, value)
    }

    fn write_f64_be(&mut self, value: f64) -> io::Result<()> {
        bitstream_io::BitWrite::write_as_from::<BE, _>(self, value)
    }

    fn write_u8_bf(&mut self, bits: u32, value: u8) -> io::Result<()> {
        bitstream_io::BitWrite::write(self, bits, value)
    }

    fn write_i8_bf(&mut self, bits: u32, value: i8) -> io::Result<()> {
        bitstream_io::BitWrite::write_signed(self, bits, value)
    }

    fn write_u16_bf(&mut self, bits: u32, value: u16) -> io::Result<()> {
        bitstream_io::BitWrite::write(self, bits, value)
    }

    fn write_i16_bf(&mut self, bits: u32, value: i16) -> io::Result<()> {
        bitstream_io::BitWrite::write_signed(self, bits, value)
    }

    fn write_u32_bf(&mut self, bits: u32, value: u32) -> io::Result<()> {
        bitstream_io::BitWrite::write(self, bits, value)
    }

    fn write_i32_bf(&mut self, bits: u32, value: i32) -> io::Result<()> {
        bitstream_io::BitWrite::write_signed(self, bits, value)
    }

    fn write_u64_bf(&mut self, bits: u32, value: u64) -> io::Result<()> {
        bitstream_io::BitWrite::write(self, bits, value)
    }

    fn write_i64_bf(&mut self, bits: u32, value: i64) -> io::Result<()> {
        bitstream_io::BitWrite::write_signed(self, bits, value)
    }
}
