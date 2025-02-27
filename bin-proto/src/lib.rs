//! Simple & fast bit-level binary co/dec in Rust.
//!
//! For more information about `#[derive(BitDecode, BitEncode)]` and its attributes, see
//! [macro@BitDecode] or [macro@BitEncode].
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! # use bin_proto::{BitDecode, BitEncode, BitCodec};
//! #[derive(Debug, BitDecode, BitEncode, PartialEq)]
//! #[codec(discriminant_type = u8)]
//! #[codec(bits = 4)]
//! enum E {
//!     V1 = 1,
//!     #[codec(discriminant = 4)]
//!     V4,
//! }
//!
//! #[derive(Debug, BitDecode, BitEncode, PartialEq)]
//! struct S {
//!     #[codec(bits = 1)]
//!     bitflag: bool,
//!     #[codec(bits = 3)]
//!     bitfield: u8,
//!     enum_: E,
//!     #[codec(write_value = self.arr.len() as u8)]
//!     arr_len: u8,
//!     #[codec(tag = arr_len as usize)]
//!     arr: Vec<u8>,
//!     #[codec(tag_type = u16, tag_value = self.prefixed_arr.len() as u16)]
//!     prefixed_arr: Vec<u8>,
//!     #[codec(flexible_array_member)]
//!     read_to_end: Vec<u8>,
//! }
//!
//! assert_eq!(
//!     S::decode_bytes(&[
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
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(rustdoc_internals))]
#![cfg_attr(docsrs, allow(internal_features))]
#![no_std]
#![deny(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    unsafe_code
)]
#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use self::bit_read::BitRead;
pub use self::bit_write::BitWrite;
pub use self::byte_order::ByteOrder;
pub use self::codec::BitCodec;
pub use self::codec::{BitDecode, BitEncode};
pub use self::discriminable::Discriminable;
pub use self::error::{Error, Result};

/// Derive the `BitDecode` and `BitEncode` traits.
///
/// # Attributes
///
/// ## `#[codec(discriminant_type = <type>)]`
/// - Applies to: `enum`
/// - `<type>`: an arbitrary type that implements `BitDecode` or `BitEncode`
///
/// Specify if enum variant should be determined by a string or interger representation of its
/// discriminant.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(discriminant_type = u8)]
/// enum Example {
///     Variant1 = 1,
///     Variant5 = 5,
/// }
/// ```
///
/// ## `#[codec(discriminant = <value>)]`
/// - Applies to: `enum` variant
/// - `<value>`: unique value of the discriminant's type
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(discriminant_type = u8)]
/// enum Example {
///     #[codec(discriminant = 1)]
///     Variant1,
///     Variant5 = 5,
/// }
/// ```
///
/// Specify the discriminant for a variant.
///
/// ## `#[codec(bits = <width>)]`
/// - Applies to: `impl BitFieldRead`, `impl BitFieldWrite`, `enum` with discriminant that
///   `impl BitField`
///
/// Determine width of field in bits.
///
/// **WARNING**: Bitfields disregard `ByteOrder` and instead have the same endianness as the
/// underlying `BitRead` / `BitWrite` instance. If you're using bitfields, you almost always want a
/// big endian stream.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Nibble(#[codec(bits = 4)] u8);
/// ```
///
/// ## `#[codec(flexible_array_member)]`
/// - Applies to: `impl UntaggedRead`
///
/// Variable-length field is final field in container, hence lacks a length prefix and should be
/// read until eof.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct ReadToEnd(#[codec(flexible_array_member)] Vec<u8>);
/// ```
///
/// ## `#[codec(tag = <expr>)]`
/// - Applies to: `impl TaggedRead` or `impl UntaggedWrite`
/// - `<expr>`: arbitrary expression. Fields in parent container can be used
///   without prefixing them with `self`.
///
/// Specify tag of field. The tag represents a length prefix for variable-length fields, and a
/// boolean for `Option`.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// pub struct WithElementsLength {
///     pub count: u32,
///     pub foo: bool,
///     #[codec(tag = count as usize)]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `#[codec(tag_type = <type>[, tag_value = <expr>]?)]`
/// - Applies to: `impl TaggedRead` or `impl UntaggedWrite`
/// - `<type>`: tag's type
/// - `<expr>`: arbitrary expression. Fields in parent container should be
///   prefixed with `self`.
///
/// Specify tag of field. The tag represents a length prefix for variable-length fields, and a
/// boolean for `Option`. The tag is placed directly before the field. The `tag_value` only has
/// to be specified when deriving `BitEncode`.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// pub struct WithElementsLength {
///     #[codec(tag_type = u16, tag_value = self.data.len() as u16)]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `#[codec(write_value = <expr>)]`
/// - Applies to: fields
/// - `<expr>`: An expression that can be coerced to the field type, potentially using `self`
///
/// Specify an expression that should be used as the field's value for writing.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// pub struct WithElementsLengthAuto {
///     #[codec(write_value = self.data.len() as u32)]
///     pub count: u32,
///     pub foo: bool,
///     #[codec(tag = count as usize)]
///     pub data: Vec<u32>,
/// }
/// ```
///
/// ## `[#codec(ctx = <type>)[, ctx_generics(<generic>[, <generic>]*)]?]`
/// - Applies to: containers
/// - `<type>`: The type of the context. Either a concrete type, or one of the container's generics
/// - `<generic>`: Any generics used by the context type, with optional bounds. E.g.
///   `T: Copy` for a `Vec<T>` context.
///
/// Specify the type of context that will be passed to codec functions.
///
/// ```
/// # use bin_proto::{ByteOrder, BitDecode, BitEncode};
/// pub struct Ctx;
///
/// pub struct NeedsCtx;
///
/// impl BitDecode<Ctx> for NeedsCtx {
///     fn decode(
///         _read: &mut dyn bin_proto::BitRead,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<Self> {
///         // Use ctx here
///         Ok(Self)
///     }
/// }
///
/// impl BitEncode<Ctx> for NeedsCtx {
///     fn encode(
///         &self,
///         _write: &mut dyn bin_proto::BitWrite,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<()> {
///         // Use ctx here
///         Ok(())
///     }
/// }
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx)]
/// pub struct WithCtx(NeedsCtx);
///
/// WithCtx(NeedsCtx)
///     .encode_bytes_ctx(ByteOrder::LittleEndian, &mut Ctx, ())
///     .unwrap();
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// # use std::marker::PhantomData;
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx)]
/// pub struct NestedCodec<Ctx, A: BitDecode<Ctx> + BitEncode<Ctx>>(A, PhantomData<Ctx>);
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// pub struct Ctx<'a, T: Copy>(&'a T);
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx<'a, T>, ctx_generics('a, T: Copy))]
/// pub struct WithCtx;
/// ```
///
/// ## `[#codec(ctx_bounds(<bound>[, <bound>]*)[, ctx_generics(<generic>[, <generic>]*)]?)]`
/// - Applies to: containers
/// - `<bounds>`: Trait bounds that must be satisfied by the context
/// - `<generic>`: Any generics used by the context type. E.g. `'a` for a context with a
///   `From<&'a i32>` bound.
///
/// Specify the trait bounds of context that will be passed to codec functions.
///
/// ```
/// # use bin_proto::{ByteOrder, BitDecode, BitEncode};
/// pub trait CtxTrait {};
///
/// pub struct NeedsCtx;
///
/// impl<Ctx: CtxTrait> BitDecode<Ctx> for NeedsCtx {
///     fn decode(
///         _read: &mut dyn bin_proto::BitRead,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<Self> {
///         // Use ctx here
///         Ok(Self)
///     }
///}
///
/// impl<Ctx: CtxTrait> BitEncode<Ctx> for NeedsCtx {
///     fn encode(
///         &self,
///         _write: &mut dyn bin_proto::BitWrite,
///         _byte_order: bin_proto::ByteOrder,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<()> {
///         // Use ctx here
///         Ok(())
///     }
/// }
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx_bounds(CtxTrait))]
/// pub struct WithCtx(NeedsCtx);
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx_bounds(From<&'a i32>), ctx_generics('a))]
/// pub struct WithCtx;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[cfg(feature = "derive")]
pub use bin_proto_derive::{BitDecode, BitEncode};

#[macro_use]
mod codec;

mod bit_read;
mod bit_write;
mod byte_order;
mod discriminable;
mod error;
mod impls;
mod util;

pub extern crate bitstream_io;

pub struct Untagged;
pub struct Tag<T>(pub T);
pub struct Bits(pub u32);

/// ```compile_fail
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct MutuallyExclusiveAttrs {
///     pub length: u8,
///     #[codec(flexible_array_member)]
///     #[codec(tag = length as usize)]
///     pub reason: String,
/// }
/// ```
#[cfg(all(feature = "derive", doctest))]
#[allow(unused)]
fn compile_fail_if_multiple_exclusive_attrs() {}
