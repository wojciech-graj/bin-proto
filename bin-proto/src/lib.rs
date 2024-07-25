//! Simple & fast bit-level binary co/dec in Rust.
//!
//! For more information about `#[derive(Protocol)]` and its attributes, see [macro@Protocol].
//!
//! # Example
//!
//! ```
//! # use bin_proto::{Protocol, ProtocolNoCtx};
//! #[derive(Debug, Protocol, PartialEq)]
//! #[protocol(discriminant_type = "u8")]
//! #[protocol(bits = 4)]
//! enum E {
//!     V1 = 1,
//!     #[protocol(discriminant = "4")]
//!     V4,
//! }
//!
//! #[derive(Debug, Protocol, PartialEq)]
//! struct S {
//!     #[protocol(bits = 1)]
//!     bitflag: bool,
//!     #[protocol(bits = 3)]
//!     bitfield: u8,
//!     enum_: E,
//!     #[protocol(write_value = "self.arr.len() as u8")]
//!     arr_len: u8,
//!     #[protocol(tag = "arr_len as usize")]
//!     arr: Vec<u8>,
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
//!         0x01, 0x02, 0x03, // read_to_end: [0x01, 0x02, 0x03]
//!     ], bin_proto::ByteOrder::BigEndian).unwrap(),
//!     S {
//!         bitflag: true,
//!         bitfield: 5,
//!         enum_: E::V1,
//!         arr_len: 2,
//!         arr: vec![0x21, 0x37],
//!         read_to_end: vec![0x01, 0x02, 0x03],
//!     }
//! );
//! ```

pub use self::bit_field::BitField;
pub use self::bit_read::BitRead;
pub use self::bit_write::BitWrite;
pub use self::byte_order::ByteOrder;
pub use self::error::{Error, Result};
pub use self::externally_tagged::ExternallyTagged;
pub use self::flexible_array_member::FlexibleArrayMember;
pub use self::protocol::Protocol;
pub use self::protocol::ProtocolNoCtx;

/// Derive the `Protocol` trait.
///
/// # Attributes
///
/// ## `#[protocol(discriminant_type = "<type>")]`
/// - Applies to: `enum`
/// - `<type>`: an arbitrary type that implements `Protocol`
///
/// Specify if enum variant should be determined by a string or interger
/// representation of its discriminant.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// #[protocol(discriminant_type = "u8")]
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
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// #[protocol(discriminant_type = "u8")]
/// enum Example {
///     #[protocol(discriminant = "1")]
///     Variant1,
///     Variant5 = 5,
/// }
/// ```
///
/// Specify the discriminant for a variant.
///
/// ## `#[protocol(bits = <width>)]`
/// - Applies to: `impl BitField`, `enum` with discriminant that `impl BitField`
///
/// Determine width of field in bits.
///
/// **WARNING**: Bitfields disregard ByteOrder and instead have the same
/// endianness as the underlying `BitRead` / `BitWrite` instance. If you're
/// using bitfields, you almost always want a big endian stream.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// struct Nibble(#[protocol(bits = 4)] u8);
/// ```
///
/// ## `#[protocol(flexible_array_member)]`
/// - Applies to: `impl FlexibleArrayMember`
///
/// Variable-length field is final field in container, hence lacks a length
/// prefix and should be read until eof.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// struct ReadToEnd(#[protocol(flexible_array_member)] Vec<u8>);
/// ```
///
/// ## `#[protocol(tag = "<expr>")]`
/// - Applies to: `impl ExternallyTagged`
/// - `<expr>`: arbitrary `usize` expression. Fields in parent container can be
///   used without prefixing them with `self`.
///
/// Specify length of variable-length field.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// pub struct WithElementsLength {
///     pub count: u32,
///     pub foo: bool,
///     #[protocol(tag = "count as usize")]
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
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// pub struct WithElementsLengthAuto {
///     #[protocol(write_value = "self.data.len() as u32")]
///     pub count: u32,
///     pub foo: bool,
///     #[protocol(tag = "count as usize")]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `[#protocol(ctx = "<ty>")]`
/// - Applies to: containers
/// - `<ty>`: The type of the context. Either a concrete type, or one of the
///   container's generics
///
/// Specify the type of context that will be passed to codec functions
///
/// ```
/// # use bin_proto::{ByteOrder, Protocol};
/// pub struct Ctx;
///
/// pub struct NeedsCtx;
///
/// impl Protocol<Ctx> for NeedsCtx {
///     fn read(
///         _read: &mut dyn bin_proto::BitRead,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///     ) -> bin_proto::Result<Self> {
///         // Use ctx here
///         Ok(Self)
///     }
///
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
/// #[derive(Protocol)]
/// #[protocol(ctx = "Ctx")]
/// pub struct WithCtx(NeedsCtx);
///
/// WithCtx(NeedsCtx)
///     .bytes_ctx(ByteOrder::LittleEndian, &mut Ctx)
///     .unwrap();
/// ```
///
/// ```
/// # use bin_proto::Protocol;
/// # use std::marker::PhantomData;
/// #[derive(Protocol)]
/// #[protocol(ctx = "Ctx")]
/// pub struct NestedProtocol<Ctx, A: Protocol<Ctx>>(A, PhantomData<Ctx>);
/// ```
///
/// ## `[#protocol(ctx_bounds = "<bounds>")]`
/// - Applies to: containers
/// - `<bounds>`: Trait bounds that must be satisfied by the context
///
/// Specify the trait bounds of context that will be passed to codec functions
///
/// ```
/// # use bin_proto::{ByteOrder, Protocol};
/// pub trait CtxTrait {};
///
/// pub struct NeedsCtx;
///
/// impl<Ctx: CtxTrait> Protocol<Ctx> for NeedsCtx {
///     fn read(
///         _read: &mut dyn bin_proto::BitRead,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///     ) -> bin_proto::Result<Self> {
///         // Use ctx here
///         Ok(Self)
///     }
///
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
/// #[derive(Protocol)]
/// #[protocol(ctx_bounds = "CtxTrait")]
/// pub struct WithCtx(NeedsCtx);
/// ```
#[cfg(feature = "derive")]
pub use bin_proto_derive::Protocol;

mod bit_field;
mod bit_read;
mod bit_write;
#[macro_use]
mod externally_tagged;
mod byte_order;
mod error;
mod flexible_array_member;
mod types;
#[macro_use]
mod protocol;
mod util;

pub extern crate bitstream_io;

/// ```compile_fail
/// #[derive(bin_proto::Protocol)]
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
