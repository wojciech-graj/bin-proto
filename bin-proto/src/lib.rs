//! Conversion to/from binary for arbitrary types. With `no_std` and `no_alloc` support.
//!
//! For more information about `#[derive(BitDecode, BitEncode)]` and its attributes, see
//! [`macro@BitDecode`] or [`macro@BitEncode`].
//!
//! # Example
//!
//! ```
//! # #[cfg(all(feature = "derive", feature = "alloc"))]
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
//!     ], bin_proto::BigEndian).unwrap().0,
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
//!
//! # Manual Implementations
//!
//! The [`macro@BitDecode`] and [`macro@BitEncode`] derive macros support the most common use-cases,
//! but it may sometimes be necessary to manually implement [`BitEncode`] or [`BitDecode`]. Both
//! traits have two generic parameters:
//! - `Ctx`: A mutable variable passed recursively down the codec chain
//! - `Tag`: A tag for specifying additional behavior
//!
//! `Tag` can have any type. The following are used throughout `bin-proto` and ensure
//! interoperability:
//! - [`Tag`]: Specifies that an additional tag is required during decoding, such as a length prefix
//!   for a [`Vec`](::alloc::vec::Vec), or a discriminant of an `enum`
//! - [`Untagged`]: Specifies that the type has a tag used during decoding, but this tag is not
//!   written during encoding
//! - [`Bits`]: Specified that the type is a bitfield, and can have a variable number of bits

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(rustdoc_internals))]
#![cfg_attr(docsrs, allow(internal_features))]
#![no_std]
#![deny(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use self::codec::BitCodec;
pub use self::codec::{BitDecode, BitDecodeExt, BitEncode, BitEncodeExt};
pub use self::discriminable::Discriminable;
pub use self::error::{Error, Result};
pub use bitstream_io::{BigEndian, BitRead, BitWrite, Endianness, LittleEndian};

/// Derive the [`BitDecode`] and [`BitEncode`] traits.
///
/// # Scopes
///
/// ```ignore
/// #[container, enum]
/// enum Enum {
///     #[variant]
///     Variant {
///         #[field]
///         field: Type,
///     }
/// }
///
/// #[container, struct]
/// struct Struct {
///     #[field]
///     field: Type,
/// }
/// ```
///
/// # Using Context
///
/// The [`ctx`](#ctx) and [`ctx_bounds`](#ctx_bounds) attributes can be used to specify additional
/// context used during codec. The context can be accessed as `__ctx`, and the tag as `__tag` in any
/// attribute macro's `<expr>`, for example in [`tag`](#tag).
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # use bin_proto::{BitDecode, BitEncode};
/// struct Ctx {
///     n: u32,
/// }
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx)]
/// struct WithElementsLength {
///     count: u32,
///     foo: bool,
///     #[codec(tag = count * __ctx.n)]
///     data: Vec<u32>,
/// }
/// # }
/// ```
///
/// # Attributes
///
/// | Attribute | Scope | Applicability |
/// |-|-|-|
/// | [`discriminant_type`](#discriminant_type) | enum | rw |
/// | [`discriminant`](#discriminant) | variant | rw |
/// | [`other`](#other) | variant | r |
/// | [`bits`](#bits) | field, enum | rw |
/// | [`flexible_array_member`](#flexible_array_member) | field | rw |
/// | [`tag`](#tag) | field | rw |
/// | [`tag_type`](#tag_type) | field | rw |
/// | [`write_value`](#write_value) | field | w |
/// | [`ctx`](#ctx) | container | rw |
/// | [`ctx_bounds`](#ctx_bounds) | container | rw |
/// | [`skip_encode`](#skip_encode) | field, variant | w |
/// | [`skip_decode`](#skip_decode) | field, variant | r |
/// | [`skip`](#skip) | field, variant | rw |
/// | [`pad_before`](#pad_before) | field, struct | rw |
/// | [`pad_after`](#pad_after) | field, struct | rw |
/// | [`magic`](#magic) | field, struct | rw |
///
/// ## `discriminant_type`
/// `#[codec(discriminant_type = <type>)]`
/// - `<type>`: an arbitrary type that implements [`BitDecode`] or [`BitEncode`]
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
/// ## `discriminant`
/// `#[codec(discriminant = <value>)]`
/// - `<value>`: unique value of the discriminant's type
///
/// Specify the discriminant for a variant.
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
/// ## `other`
/// `#[codec(other)]`
///
/// Decode the specified variant if the discriminant doesn't match any other variants. A
/// discriminant value can still be provided for the variant, and will be used when encoding.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(discriminant_type = u8)]
/// enum Example {
///     #[codec(discriminant = 1)]
///     Variant1,
///     #[codec(discriminant = 2, other)]
///     CatchAll,
/// }
/// ```
///
/// ## `bits`
/// `#[codec(bits = <width>)]`
///
/// Determine width of field in bits.
///
/// **WARNING**: Bitfields disregard endianness and instead have the same endianness as the
/// underlying [`BitRead`] / [`BitWrite`] instance. If you're using bitfields, you almost always
/// want a big endian stream.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Nibble(#[codec(bits = 4)] u8);
/// ```
///
/// ## `flexible_array_member`
/// `#[codec(flexible_array_member)]`
///
/// Variable-length field is final field in container, hence lacks a length prefix and should be
/// read until eof.
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct ReadToEnd(#[codec(flexible_array_member)] Vec<u8>);
/// # }
/// ```
///
/// ## `tag`
/// `#[codec(tag = <expr>)]`
/// - `<expr>`: arbitrary expression. Fields in parent container can be used without prefixing them
///   with `self`.
///
/// Specify tag of field. The tag represents a length prefix for variable-length fields, and a
/// boolean for [`Option`].
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct WithElementsLength {
///     count: u32,
///     foo: bool,
///     #[codec(tag = count as usize)]
///     data: Vec<u32>,
/// }
/// # }
/// ```
///
/// ## `tag_type`
/// `#[codec(tag_type = <type>[, tag_value = <expr>]?[, tag_bits = <expr>]?)]`
/// - `<type>`: tag's type
/// - `<expr>`: arbitrary expression. Fields in parent container should be prefixed with `self`.
///
/// Specify tag of field. The tag represents a length prefix for variable-length fields, and a
/// boolean for [`Option`]. The tag is placed directly before the field. The `tag_value` only has
/// to be specified when deriving [`BitEncode`].
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct WithElementsLength {
///     #[codec(tag_type = u16, tag_value = self.data.len() as u16, tag_bits = 13)]
///     data: Vec<u32>,
/// }
/// # }
/// ```
///
/// ## `write_value`
/// `#[codec(write_value = <expr>)]`
/// - `<expr>`: An expression that can be coerced to the field type. Fields in parent container
///   should be prefixed with `self`.
///
/// Specify an expression that should be used as the field's value for writing.
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct WithElementsLengthAuto {
///     #[codec(write_value = self.data.len() as u32)]
///     count: u32,
///     foo: bool,
///     #[codec(tag = count as usize)]
///     data: Vec<u32>,
/// }
/// # }
/// ```
///
/// ## `ctx`
/// `#[codec(ctx = <type>)[, ctx_generics(<generic>[, <generic>]*)]?]`
/// - `<type>`: The type of the context. Either a concrete type, or one of the container's generics
/// - `<generic>`: Any generics used by the context type, with optional bounds. E.g.
///   `T: Copy` for a [`Vec<T>`](alloc::vec::Vec) context.
///
/// Specify the type of context that will be passed to codec functions.
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # use bin_proto::{BitDecode, BitEncode, BitEncodeExt};
/// struct Ctx;
///
/// struct NeedsCtx;
///
/// impl BitDecode<Ctx> for NeedsCtx {
///     fn decode<R, E>(
///         _read: &mut R,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<Self>
///     where
///         R: bin_proto::BitRead,
///         E: bin_proto::Endianness,
///     {
///         // Use ctx here
///         Ok(Self)
///     }
/// }
///
/// impl BitEncode<Ctx> for NeedsCtx {
///     fn encode<W, E>(
///         &self,
///         _write: &mut W,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<()>
///     where
///         W: bin_proto::BitWrite,
///         E: bin_proto::Endianness,
///     {
///         // Use ctx here
///         Ok(())
///     }
/// }
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx)]
/// struct WithCtx(NeedsCtx);
///
/// WithCtx(NeedsCtx)
///     .encode_bytes_ctx(bin_proto::BigEndian, &mut Ctx, ())
///     .unwrap();
/// # }
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// # use std::marker::PhantomData;
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx)]
/// struct NestedCodec<Ctx, A: BitDecode<Ctx> + BitEncode<Ctx>>(A, PhantomData<Ctx>);
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// struct Ctx<'a, T: Copy>(&'a T);
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx = Ctx<'a, T>, ctx_generics('a, T: Copy))]
/// struct WithCtx;
/// ```
///
/// ## `ctx_bounds`
/// `#[codec(ctx_bounds(<bound>[, <bound>]*)[, ctx_generics(<generic>[, <generic>]*)]?)]`
/// - `<bounds>`: Trait bounds that must be satisfied by the context
/// - `<generic>`: Any generics used by the context type. E.g. `'a` for a context with a
///   [`From<&'a i32>`](From) bound.
///
/// Specify the trait bounds of context that will be passed to codec functions.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// trait CtxTrait {};
///
/// struct NeedsCtx;
///
/// impl<Ctx: CtxTrait> BitDecode<Ctx> for NeedsCtx {
///     fn decode<R, E>(
///         _read: &mut R,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<Self>
///     where
///         R: bin_proto::BitRead,
///         E: bin_proto::Endianness,
///     {
///         // Use ctx here
///         Ok(Self)
///     }
///}
///
/// impl<Ctx: CtxTrait> BitEncode<Ctx> for NeedsCtx {
///     fn encode<W, E>(
///         &self,
///         _write: &mut W,
///         _ctx: &mut Ctx,
///         _tag: (),
///     ) -> bin_proto::Result<()>
///     where
///         W: bin_proto::BitWrite,
///         E: bin_proto::Endianness,
///     {
///         // Use ctx here
///         Ok(())
///     }
/// }
///
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx_bounds(CtxTrait))]
/// struct WithCtx(NeedsCtx);
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(ctx_bounds(From<&'a i32>), ctx_generics('a))]
/// struct WithCtx;
/// ```
///
/// ## `skip_encode`
/// `#[codec(skip_encode)]`
///
/// If applied to a field, skip the field when encoding. If applied to an enum variant, return an
/// Error if the variant is attempted to be encoded.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Struct(#[codec(skip_encode)] u8);
/// ```
///
/// ```
/// # use bin_proto::BitEncode;
/// #[derive(BitEncode)]
/// #[codec(discriminant_type = u8)]
/// enum Enum {
///     #[codec(skip_encode)]
///     Skip
/// }
/// ```
///
/// ## `skip_decode`
/// `#[codec(skip_decode)]`
///
/// If applied to a field, use [`Default::default`] instead of attempting to read field. If applied
/// to an enum variant, don't generate code for decoding.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Struct(#[codec(skip_decode)] u8);
/// ```
///
/// ```
/// # use bin_proto::BitDecode;
/// #[derive(BitDecode)]
/// #[codec(discriminant_type = u8)]
/// enum Enum {
///     #[codec(skip_decode)]
///     Skip
/// }
/// ```
///
/// ## `skip`
/// `#[codec(skip)]`
///
/// Equivalent to combining [`skip_encode`](#skip_encode) and [`skip_decode`](#skip_decode).
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Struct(#[codec(skip)] u8);
/// ```
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(discriminant_type = u8)]
/// enum Enum {
///     #[codec(skip)]
///     Skip
/// }
/// ```
///
/// ## `pad_before`
/// `#[codec(pad_before = <expr>)]`
///
/// Insert 0 bits when writing and skip bits when reading, prior to processing the field.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Struct(#[codec(pad_before = 3)] u8);
/// ```
///
/// ## `pad_after`
/// `#[codec(pad_after = <expr>)]`
///
/// Insert 0 bits when writing and skip bits when reading, after processing the field.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct Struct(#[codec(pad_after = 3)] u8);
/// ```
///
/// ## `magic`
/// `#[codec(magic = <expr>)]`
/// - `<expr>`: Must evaluate to `&[u8; _]`
///
/// Indicates that the value must be present immediately preceding the field or struct.
///
/// ```
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// #[codec(magic = &[0x01, 0x02, 0x03])]
/// struct Magic(#[codec(magic = b"123")] u8);
/// ```
#[cfg(feature = "derive")]
pub use bin_proto_derive::{BitDecode, BitEncode};

#[macro_use]
mod codec;

mod discriminable;
mod error;
mod impls;
mod util;

pub extern crate bitstream_io;

/// A marker for [`BitEncode`] implementors that don't prepend their tag, and [`BitDecode`]
/// implementors that usually have a tag, but can be read to EOF
pub struct Untagged;

/// A marker for [`BitDecode`] implementors that require a tag.
pub struct Tag<T>(pub T);

/// A marker for [`BitDecode`] and [`BitEncode`] implementors that support bitfield operations.
pub struct Bits<const C: u32>;

/// ```compile_fail
/// # use bin_proto::{BitDecode, BitEncode};
/// #[derive(BitDecode, BitEncode)]
/// struct MutuallyExclusiveAttrs {
///     pub length: u8,
///     #[codec(flexible_array_member)]
///     #[codec(tag = length as usize)]
///     pub reason: alloc::string::String,
/// }
/// ```
#[cfg(all(feature = "derive", feature = "alloc", doctest))]
#[allow(unused)]
fn compile_fail_if_multiple_exclusive_attrs() {}
