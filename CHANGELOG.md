# v1.0.0
- Seal `BitDecodeExt`, `BitEncodeExt`, `BitCodec` traits
- Add `#[codec(other)]` attribute
- Replace `#[codec(default)]` with `#[codec(skip_encode)]`, `#[codec(skip_decode)]`, `#[codec(skip)]`
- Modify `Discriminable::discriminant` to return `Option`
- Add `&'static str` field for message to `Error::Other`
- Add `BitDecodeExt::decode_all_bytes_ctx` and `BitCodec::decode_all_bytes`
- Rename `#[codec(flexible_array_member)]` to `#[codec(untagged)]`
# v0.11.1
- Fix potential memory leak in `BitDecode` implementation for `[T; N]`
# v0.11.0
- Return count of bits read from `BitDecodeExt::decode_bytes_ctx`, `BitCodec::decode_bytes`
- Return count of bytes written from `BitEncodeExt::encode_bytes_ctx_buf`, `BitCodec::encode_bytes_buf`
- Implement `BitDecode` for `Box<CStr>`, `Box<str>`, `Box<[T]>`
# v0.10.0
- Add support for `no_alloc`
- Fix `no_std` support
- Split `BitEncode`, `BitDecode` traits into `BitEncode`, `BitEncodeExt`, `BitDecode`, `BitDecodeExt`
- Add `BitEncodeExt::encode_bytes_ctx_buf` and `BitCodec::encode_bytes_buf`
- Change `Error::Magic` variant to only contain static data
- Rename `Error::UnknownEnumDiscriminant` to `Error::Discriminant`
- Rename `Error::Other` to `Error::Boxed`
- Add `Error::Other` variant
- Optimize reading `[T; N]`
# v0.9.1
- Add `#[codec(default)]`, `#[codec(pad_before = ...)]`, `#[codec(pad_after = ...)]`, `#[codec(magic = ...)]`
- Add `Error::Magic` variant
# v0.9.0
- Change `Bits(u32)` to `Bits<const C: u32>`
- Improve `BitDecode` performance for collections
# v0.8.0
- Add `#[codec(tag_bits = ...)]` attribute
- Update `bitstream_io` to 4.0
# v0.7.1
- Expand dependency version ranges
- Bump bitstream-io to 3.1
# v0.7.0
- Combine `TaggedRead`, `FlexibleArrayMemberRead`, `BitFieldRead`, and `ProtocolRead` traits into `BitDecode`
- Combine `UntaggedWrite`, `BitFieldWrite`, and `ProtocolWrite` traits into `BitEncode`
- Replace `BitRead`, `BitWrite`, and `ByteOrder` traits with `BitRead`, `BitWrite`, `Endianness` from `bitstream_io`
- Rename `ProtocolNoCtx` to `Codec`
- Change all `protocol` attributes to `codec`
- Add `std` feature, support `no_std`
- Implement `BitDecode` and `BitEncode` on tuples with up to 16 items, `NonZeroUX`, `NonZeroIX`, `Wrapping`, `Saturating`
- Implement `BitEncode` on `CStr`, `Cow`, `Cell`, `RwLock`, `Mutex`, `RefCell`, `&T`, `&mut T`
- Implement `BitEncode<_, Untagged>` on `[T]`, `str`
- Implement `BitEncode<_, Bits>` and `BitDecode<_, Bits>` on `NonZeroUX`, `NonZeroIX`, `u64`, `i64`
- Don't use implicit hasher in `HashSet` and `HashMap` impls
- Increase MSRV to 1.83.0
- Remove `BitRead::read_unaryX` and `BitWrite::write_unaryX`
- Remove `Error` suffix from `Error` variants
- Add `Error::Borrow`, `Error::Poison`, `Error::SliceTryFromVec`
- Remove `thiserror` dependency
- Deny `unwrap`, `expect`, and `unsafe`
- Clean up tuple impl documentation with `doc(fake_variadic)`
- Update `bitstream_io` to 3.0
# v0.6.0
- Allow multiple attributes in a single `#[protocol(...)]`
- Require unquoted expressions in attributes
- Use nested metas for all lists in attributes
- Add `#[protocol(ctx_generics(...))]`
- Improve attribute parsing and validation
- Impose `non_exhaustive` on `Error`
# v0.5.0
- Split `Protocol` into `ProtocolRead` and `ProtocolWrite`
- Split `ExternallyLengthPrefixed` into `TaggedRead` and `UntaggedWrite`
- Convert `FlexibleArrayMember` to `FlexibleArrayMemberRead`
- Split `BitField` into `BitFieldWrite` and `BitFieldRead`
- Implement `TaggedRead`, `UntaggedWrite` `FlexibleArrayMemberRead` on all list and map types and `Option`
- Add `Error` variant for failed `TryFrom` conversion for `TaggedRead` tags
- Add generic `Tag` parameter to `TaggedRead`
- Allow for `#[protocol(tag(type = "<type>", write_value = "<expr>"))]` attribute macro
- Unimplement `ProtocolRead` and `BitFieldRead` on `Option`
- Create `Discriminable` trait for obtaining `enum` discriminants
- Additionally derive `Discriminable`, `TaggedRead`, `UntaggedWrite`
- Implement number-related traits on `usize` and `isize`
# v0.4.2
- Set MSRV at 1.63.0
# v0.4.1
- Impose `Send + Sync` bounds on `Error::Other`
# v0.4.0
- Delete `EnumExt`
- Bump dependencies, and rust version to 2021
- Make lifetime generics work
- Handle context using generics instead of `Any`
- Improve derive macro hygiene
- Improve derive macro error reporting
# v0.3.4
- Do not trigger https://github.com/rust-lang/rust/issues/120363 with generated code
# v0.3.3
- Add `Other` error type
# v0.3.2
- Use `std::net` instead of `core::net`
# v0.3.1
- Implement `Protocol` on `i128`, `u128`, `PhantomPinned`, `BinaryHeap`
- Fix `length` attribute not working in enum variant
# v0.3.0
- Implement `ExternallyLengthPrefixed` on `HashMap`, `BTreeMap`, `String`
- Do not implement `Protocol` on `char`, `range`, `HashMap`, `BTreeMap`
- Implement protocol on `Ipv4Addr` and `Ipv6Addr`, `(T0, )`, `()`, `Box`
- Rename `Enum` trait to `EnumExt`
- Delete `Settings`, replace with `ByteOrder`
- Clean up `Error`
# v0.2.0
- Add context to all parse functions
- Remove `#[repr(...)]`, instead specify repr in `#[protocol(discriminant = "...")]`
- Remove Hints, LengthPrefixed, etc.
- Add `#[protocol(write_value = "<expr>")]` for automatically writing arbitrary element value
- Replace `#[protocol(length_prefix(<kind>(<field>)))]` with `#[protocol(length = "<expr>")]`
- Check attribute applicability in every context
- Require discriminants type to be specified for an enum
