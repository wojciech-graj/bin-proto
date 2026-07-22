//! A large untrusted length prefix must not force a huge up-front allocation or a
//! capacity-overflow panic before a single element has been read from the stream.
#![cfg(feature = "std")]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use bin_proto::{BigEndian, BitDecodeExt, Tag};

// `usize::MAX` elements can never be read from an empty stream, so decoding must
// return an error rather than reserve capacity for all of them up front (which
// either panics with `capacity overflow` or attempts a multi-GB reservation).
macro_rules! assert_hostile_len_errors {
    ($ty:ty) => {{
        let empty: &[u8] = &[];
        let res = <$ty>::decode_bytes_ctx(empty, BigEndian, &mut (), Tag(usize::MAX));
        assert!(
            res.is_err(),
            "{} did not error on a hostile length prefix",
            stringify!($ty)
        );
    }};
}

#[test]
fn hostile_length_prefix_errors_not_panics() {
    assert_hostile_len_errors!(Vec<u8>);
    assert_hostile_len_errors!(Vec<u128>);
    assert_hostile_len_errors!(VecDeque<u8>);
    assert_hostile_len_errors!(BinaryHeap<u8>);
    assert_hostile_len_errors!(HashSet<u8>);
    assert_hostile_len_errors!(HashMap<u8, u8>);
    assert_hostile_len_errors!(String);
    // Delegate to Vec / String decode, so they inherit the same fix.
    assert_hostile_len_errors!(Box<[u8]>);
    assert_hostile_len_errors!(Box<str>);
    // Siblings that already used `::new()` (no pre-alloc); kept here as a guard.
    assert_hostile_len_errors!(LinkedList<u8>);
    assert_hostile_len_errors!(BTreeSet<u8>);
    assert_hostile_len_errors!(BTreeMap<u8, u8>);
}

#[test]
fn legitimate_decode_is_unchanged() {
    let (v, _) = Vec::<u8>::decode_bytes_ctx(&[1, 2, 3], BigEndian, &mut (), Tag(3)).unwrap();
    assert_eq!(v, vec![1, 2, 3]);
    let (s, _) = String::decode_bytes_ctx(b"abc", BigEndian, &mut (), Tag(3)).unwrap();
    assert_eq!(s, "abc");
}
