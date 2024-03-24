use std::io;

use bitstream_io::{BE, LE};

pub trait BitRead {
    fn read_bit(&mut self) -> io::Result<bool>;
    fn skip(&mut self, bits: u32) -> io::Result<()>;
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()>;
    fn read_to_vec(&mut self, bytes: usize) -> io::Result<Vec<u8>>;
    fn read_unary0(&mut self) -> io::Result<u32>;
    fn read_unary1(&mut self) -> io::Result<u32>;
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
    fn read_f32_le(&mut self) -> io::Result<f32>;
    fn read_f32_be(&mut self) -> io::Result<f32>;
    fn read_f64_le(&mut self) -> io::Result<f64>;
    fn read_f64_be(&mut self) -> io::Result<f64>;

    fn read_u(&mut self, bits: u32) -> io::Result<u8>;
    fn read_i(&mut self, bits: u32) -> io::Result<i8>;
}

// TODO(wgraj): MACROFY THIS
impl<T: bitstream_io::BitRead> BitRead for T {
    fn read_bit(&mut self) -> io::Result<bool> {
        bitstream_io::BitRead::read_bit(self)
    }

    fn skip(&mut self, bits: u32) -> io::Result<()> {
        bitstream_io::BitRead::skip(self, bits)
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        bitstream_io::BitRead::read_bytes(self, buf)
    }

    fn read_to_vec(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        bitstream_io::BitRead::read_to_vec(self, bytes)
    }

    fn read_unary0(&mut self) -> io::Result<u32> {
        bitstream_io::BitRead::read_unary0(self)
    }

    fn read_unary1(&mut self) -> io::Result<u32> {
        bitstream_io::BitRead::read_unary1(self)
    }

    fn byte_aligned(&self) -> bool {
        bitstream_io::BitRead::byte_aligned(self)
    }

    fn byte_align(&mut self) {
        bitstream_io::BitRead::byte_align(self)
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

    fn read_u(&mut self, bits: u32) -> io::Result<u8> {
        if bits <= 8 {
            bitstream_io::BitRead::read(self, bits)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot read > 8 bits.",
            ))
        }
    }

    fn read_i(&mut self, bits: u32) -> io::Result<i8> {
        if bits <= 8 {
            bitstream_io::BitRead::read_signed(self, bits)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot read > 8 bits.",
            ))
        }
    }
}
