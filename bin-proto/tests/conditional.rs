use bin_proto::{ByteOrder, Protocol, ProtocolNoCtx};

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub reason_length: u8,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct WithCondition {
    pub foo: u32,
    #[protocol(condition = "foo > 3")]
    pub data: Option<u32>,
}



#[test]
fn can_read_conditional_element_true() {
    assert_eq!(
        WithCondition {
            foo: 5,
            data: Some(3u32),
        },
        WithCondition::from_bytes(
            &[
                0, 0, 0, 5, // conditional
                0, 0, 0, 3 // 3
            ],
            ByteOrder::BigEndian,
        )
        .unwrap()
    );
}
#[test]
fn can_read_conditional_element_false() {
    assert_eq!(
        WithCondition {
            foo: 2,
            data: None,
        },
        WithCondition::from_bytes(
            &[
                0, 0, 0, 2, // conditional
            ],
            ByteOrder::BigEndian,
        )
        .unwrap()
    );
}