use crate::{BitRead, BitWrite, ByteOrder, Result};

/// A trait for variable-width bit-level decoding.
///
/// **WARNING**: This trait can and often will ignore the endianness.
pub trait BitFieldRead<Ctx = ()>: Sized {
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        bits: u32,
    ) -> Result<Self>;
}

/// A trait for variable-width bit-level encoding.
///
/// **WARNING**: This trait can and often will ignore the endianness.
pub trait BitFieldWrite<Ctx = ()> {
    fn write(
        &self,
        write: &mut dyn BitWrite,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        bits: u32,
    ) -> Result<()>;
}

/// ```compile_fail
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// #[protocol(discriminant_type = "u8")]
/// #[protocol(bits = 1)]
/// enum WontFit {
///     Variant = 2,
/// }
/// ```
#[cfg(all(feature = "derive", doctest))]
#[allow(unused)]
fn compile_fail_if_discriminant_wont_fit() {}
