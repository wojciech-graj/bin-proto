# v0.3.0
- Implement `ExternallyLengthPrefixed` on `HashMap`, `BTreeMap`, `String`
- Do not implement `Protocol` on `char`, `range`, `HashMap`, `BTreeMap`
- Implement protocol on `Ipv4Addr` and `Ipv6Addr`, `(T0, )`, `()`, `Box`
- Rename `Enum` trait to `EnumExt`
# v0.2.0
- Add context to all parse functions
- Remove `#[repr(...)]`, instead specify repr in `#[protocol(discriminant = "...")]`
- Remove Hints, LengthPrefixed, etc.
- Add `#[protocol(write_value = "<expr>")]` for automatically writing arbitrary element value
- Replace `#[protocol(length_prefix(<kind>(<field>)))]` with `#[protocol(length = "<expr>")]`
- Check attribute applicability in every context
- Require discriminants type to be specified for an enum
