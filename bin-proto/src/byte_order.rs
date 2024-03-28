/// Endianness.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ByteOrder {
    /// Least significant byte first.
    LittleEndian,
    /// Most significant byte first.
    BigEndian,
}

macro_rules! impl_byte_order_helpers {
    ( $( $ty:ty => [ $read_name:ident : [ $read_le:ident, $read_be:ident ], $write_name:ident : [ $write_le:ident, $write_be:ident ] ] )* ) => {
        impl ByteOrder {
            $(
                pub fn $read_name(&self, read: &mut dyn crate::BitRead) -> Result<$ty, crate::Error> {
                    Ok(match *self {
                        ByteOrder::LittleEndian => crate::BitRead::$read_le(read),
                        ByteOrder::BigEndian => crate::BitRead::$read_be(read),
                    }?)
                }

                pub fn $write_name(&self, value: $ty,
                                   write: &mut dyn crate::BitWrite) -> Result<(), crate::Error> {
                    Ok(match *self {
                        ByteOrder::LittleEndian => crate::BitWrite::$write_le(write, value),
                        ByteOrder::BigEndian => crate::BitWrite::$write_be(write, value),
                    }?)
                }
            )*
        }
    };
}

impl_byte_order_helpers!(
    u16 => [read_u16 : [read_u16_le, read_u16_be], write_u16 : [write_u16_le, write_u16_be]]
    i16 => [read_i16 : [read_i16_le, read_i16_be], write_i16 : [write_i16_le, write_i16_be]]
    u32 => [read_u32 : [read_u32_le, read_u32_be], write_u32 : [write_u32_le, write_u32_be]]
    i32 => [read_i32 : [read_i32_le, read_i32_be], write_i32 : [write_i32_le, write_i32_be]]
    u64 => [read_u64 : [read_u64_le, read_u64_be], write_u64 : [write_u64_le, write_u64_be]]
    i64 => [read_i64 : [read_i64_le, read_i64_be], write_i64 : [write_i64_le, write_i64_be]]
    f32 => [read_f32 : [read_f32_le, read_f32_be], write_f32 : [write_f32_le, write_f32_be]]
    f64 => [read_f64 : [read_f64_le, read_f64_be], write_f64 : [write_f64_le, write_f64_be]]
);
