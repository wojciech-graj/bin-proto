# v0.5.1
- Make `write_value` field in `#[protocol(tag(...))]` only required when deriving `ProtocolWrite`
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
