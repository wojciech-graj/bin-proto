//! Simple & fast bit-level binary co/dec in Rust.
//!
//! For more information about `#[derive(ProtocolRead, ProtocolWrite)]` and its attributes, see [macro@ProtocolRead] or [macro@ProtocolWrite].
//!
//! # Example
//!
//! ```
//! # use bin_proto::{ProtocolRead, ProtocolWrite, ProtocolNoCtx};
//! #[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
//! #[protocol(discriminant_type = u8)]
//! #[protocol(bits = 4)]
//! enum E {
//!     V1 = 1,
//!     #[protocol(discriminant = 4)]
//!     V4,
//! }
//!
//! #[derive(Debug, ProtocolRead, ProtocolWrite, PartialEq)]
//! struct S {
//!     #[protocol(bits = 1)]
//!     bitflag: bool,
//!     #[protocol(bits = 3)]
//!     bitfield: u8,
//!     enum_: E,
//!     #[protocol(write_value = self.arr.len() as u8)]
//!     arr_len: u8,
//!     #[protocol(tag = arr_len as usize)]
//!     arr: Vec<u8>,
//!     #[protocol(tag_type = u16, tag_value = self.prefixed_arr.len() as u16)]
//!     prefixed_arr: Vec<u8>,
//!     #[protocol(flexible_array_member)]
//!     read_to_end: Vec<u8>,
//! }
//!
//! assert_eq!(
//!     S::from_bytes(&[
//!         0b1000_0000 // bitflag: true (1)
//!        | 0b101_0000 // bitfield: 5 (101)
//!            | 0b0001, // enum_: V1 (0001)
//!         0x02, // arr_len: 2
//!         0x21, 0x37, // arr: [0x21, 0x37]
//!         0x00, 0x01, 0x33, // prefixed_arr: [0x33]
//!         0x01, 0x02, 0x03, // read_to_end: [0x01, 0x02, 0x03]
//!     ], bin_proto::ByteOrder::BigEndian).unwrap(),
//!     S {
//!         bitflag: true,
//!         bitfield: 5,
//!         enum_: E::V1,
//!         arr_len: 2,
//!         arr: vec![0x21, 0x37],
//!         prefixed_arr: vec![0x33],
//!         read_to_end: vec![0x01, 0x02, 0x03],
//!     }
//! );
//! ```

#![deny(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::implicit_hasher
)]
pub use self::bit_field::{BitFieldRead, BitFieldWrite};
pub use self::bit_read::BitRead;
pub use self::bit_write::BitWrite;
pub use self::byte_order::ByteOrder;
pub use self::discriminable::Discriminable;
pub use self::error::{Error, Result};
pub use self::flexible_array_member::FlexibleArrayMemberRead;
pub use self::protocol::ProtocolNoCtx;
pub use self::protocol::{ProtocolRead, ProtocolWrite};
pub use self::tagged::{TaggedRead, UntaggedWrite};

/// Derive the `ProtocolRead` and `ProtocolWrite` traits.
///
/// # Attributes
///
/// ## `#[protocol(discriminant_type = "<type>")]`
/// - Applies to: `enum`
/// - `<type>`: an arbitrary type that implements `ProtocolRead` or `ProtocolWrite`
///
/// Specify if enum variant should be determined by a string or interger
/// representation of its discriminant.
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// #[protocol(discriminant_type = u8)]
/// enum Example {
///     Variant1 = 1,
///     Variant5 = 5,
/// }
/// ```
///
/// ## `#[protocol(discriminant = "<value>")]`
/// - Applies to: `enum` variant
/// - `<value>`: unique value of the discriminant's type
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// #[protocol(discriminant_type = u8)]
/// enum Example {
///     #[protocol(discriminant = 1)]
///     Variant1,
///     Variant5 = 5,
/// }
/// ```
///
/// Specify the discriminant for a variant.
///
/// ## `#[protocol(bits = <width>)]`
/// - Applies to: `impl BitFieldRead`, `impl BitFieldWrite`, `enum` with discriminant that `impl BitField`
///
/// Determine width of field in bits.
///
/// **WARNING**: Bitfields disregard `ByteOrder` and instead have the same
/// endianness as the underlying `BitRead` / `BitWrite` instance. If you're
/// using bitfields, you almost always want a big endian stream.
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// struct Nibble(#[protocol(bits = 4)] u8);
/// ```
///
/// ## `#[protocol(flexible_array_member)]`
/// - Applies to: `impl FlexibleArrayMemberRead`
///
/// Variable-length field is final field in container, hence lacks a length
/// prefix and should be read until eof.
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// struct ReadToEnd(#[protocol(flexible_array_member)] Vec<u8>);
/// ```
///
/// ## `#[protocol(tag = "<expr>")]`
/// - Applies to: `impl TaggedRead` or `impl UntaggedWrite`
/// - `<expr>`: arbitrary expression. Fields in parent container can be used
///   without prefixing them with `self`.
///
/// Specify tag of field. The tag represents a length prefix for variable-length
/// fields, and a boolean for `Option`.
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// pub struct WithElementsLength {
///     pub count: u32,
///     pub foo: bool,
///     #[protocol(tag = count as usize)]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `#[protocol(tag(type = "<type>"[, write_value = "<expr>"]?))]`
/// - Applies to: `impl TaggedRead` or `impl UntaggedWrite`
/// - `<type>`: tag's type
/// - `<expr>`: arbitrary expression. Fields in parent container should be
///   prefixed with `self`.
///
/// Specify tag of field. The tag represents a length prefix for variable-length
/// fields, and a boolean for `Option`. The tag is placed directly before the
/// field. The `write_value` only has to be specified when deriving
/// `ProtocolWrite`.
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// pub struct WithElementsLength {
///     #[protocol(tag_type = u16, tag_value = self.data.len() as u16)]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `#[protocol(write_value = "<expr>")]`
/// - Applies to: fields
/// - `<expr>`: An expression that can be coerced to the field type, potentially
///   using `self`
///
/// Specify an expression that should be used as the field's value for writing.
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// pub struct WithElementsLengthAuto {
///     #[protocol(write_value = self.data.len() as u32)]
///     pub count: u32,
///     pub foo: bool,
///     #[protocol(tag = count as usize)]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `[#protocol(ctx = "<type>")]`
/// - Applies to: containers
/// - `<type>`: The type of the context. Either a concrete type, or one of the
///   container's generics
///
/// Specify the type of context that will be passed to codec functions
///
/// ```
/// # use bin_proto::{ByteOrder, ProtocolRead, ProtocolWrite};
/// pub struct Ctx;
///
/// pub struct NeedsCtx;
///
/// impl ProtocolRead<Ctx> for NeedsCtx {
///     fn read(
///         _read: &mut dyn bin_proto::BitRead,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///     ) -> bin_proto::Result<Self> {
///         // Use ctx here
///         Ok(Self)
///     }
/// }
///
/// impl ProtocolWrite<Ctx> for NeedsCtx {
///     fn write(
///         &self,
///         _write: &mut dyn bin_proto::BitWrite,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///     ) -> bin_proto::Result<()> {
///         // Use ctx here
///         Ok(())
///     }
/// }
///
/// #[derive(ProtocolRead, ProtocolWrite)]
/// #[protocol(ctx = Ctx)]
/// pub struct WithCtx(NeedsCtx);
///
/// WithCtx(NeedsCtx)
///     .bytes_ctx(ByteOrder::LittleEndian, &mut Ctx)
///     .unwrap();
/// ```
///
/// ```
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// # use std::marker::PhantomData;
/// #[derive(ProtocolRead, ProtocolWrite)]
/// #[protocol(ctx = Ctx)]
/// pub struct NestedProtocol<Ctx, A: ProtocolRead<Ctx> + ProtocolWrite<Ctx>>(A, PhantomData<Ctx>);
/// ```
///
/// ## `[#protocol(ctx_bounds = "<bounds>")]`
/// - Applies to: containers
/// - `<bounds>`: Trait bounds that must be satisfied by the context
///
/// Specify the trait bounds of context that will be passed to codec functions
///
/// ```
/// # use bin_proto::{ByteOrder, ProtocolRead, ProtocolWrite};
/// pub trait CtxTrait {};
///
/// pub struct NeedsCtx;
///
/// impl<Ctx: CtxTrait> ProtocolRead<Ctx> for NeedsCtx {
///     fn read(
///         _read: &mut dyn bin_proto::BitRead,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///     ) -> bin_proto::Result<Self> {
///         // Use ctx here
///         Ok(Self)
///     }
///}
///
/// impl<Ctx: CtxTrait> ProtocolWrite<Ctx> for NeedsCtx {
///     fn write(
///         &self,
///         _write: &mut dyn bin_proto::BitWrite,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///     ) -> bin_proto::Result<()> {
///         // Use ctx here
///         Ok(())
///     }
/// }
///
/// #[derive(ProtocolRead, ProtocolWrite)]
/// #[protocol(ctx_bounds = CtxTrait)]
/// pub struct WithCtx(NeedsCtx);
/// ```
#[cfg(feature = "derive")]
pub use bin_proto_derive::{ProtocolRead, ProtocolWrite};

mod bit_field;
mod bit_read;
mod bit_write;
#[macro_use]
mod tagged;
mod byte_order;
mod error;
mod flexible_array_member;
mod types;
#[macro_use]
mod protocol;
mod discriminable;
mod util;

pub extern crate bitstream_io;

/// ```compile_fail
/// # use bin_proto::{ProtocolRead, ProtocolWrite};
/// #[derive(ProtocolRead, ProtocolWrite)]
/// struct MutuallyExclusiveAttrs {
///     pub length: u8,
///     #[protocol(flexible_array_member)]
///     #[protocol(tag = "length as usize")]
///     pub reason: String,
/// }
/// ```
#[cfg(all(feature = "derive", doctest))]
#[allow(unused)]
fn compile_fail_if_multiple_exclusive_attrs() {}
