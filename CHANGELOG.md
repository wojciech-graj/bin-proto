# v0.3.0
- Implement `ExternallyLengthPrefixed` on `HashMap` and `BTreeMap` instead of implementing `Protocol`
- Do not implement `Protocol` on `char`, `range`
# v0.2.0
- Add context to all parse functions
- Remove `#[repr(...)]`, instead specify repr in `#[protocol(discriminant = "...")]`
- Remove Hints, LengthPrefixed, etc.
- Add `#[protocol(write_value = "<expr>")]` for automatically writing arbitrary element value
- Replace `#[protocol(length_prefix(<kind>(<field>)))]` with `#[protocol(length = "<expr>")]`
- Check attribute applicability in every context
- Require discriminants type to be specified for an enum
