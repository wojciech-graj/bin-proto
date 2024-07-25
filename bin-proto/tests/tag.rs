use bin_proto::{ByteOrder, ProtocolNoCtx, ProtocolRead, ProtocolWrite};

#[derive(ProtocolRead, ProtocolWrite, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub reason_length: u8,
}

#[derive(ProtocolRead, ProtocolWrite, Debug, PartialEq, Eq)]
pub struct WithElementsLength {
    pub count: u32,
    pub foo: bool,
    #[protocol(tag = "count as usize")]
    pub data: Vec<u32>,
}

#[derive(ProtocolRead, ProtocolWrite, Debug, PartialEq, Eq)]
pub struct WithElementsLengthAuto {
    #[protocol(write_value = "self.data.len() as u32")]
    pub count: u32,
    pub foo: bool,
    #[protocol(tag = "count as usize")]
    pub data: Vec<u32>,
}

#[derive(ProtocolRead, ProtocolWrite, Debug, PartialEq, Eq)]
#[protocol(discriminant_type = "u8")]
pub enum WithElementsLengthAutoEnum {
    #[protocol(discriminant = "1")]
    Variant {
        #[protocol(write_value = "data.len() as u32")]
        count: u32,
        foo: bool,
        #[protocol(tag = "count as usize")]
        data: Vec<u32>,
    },
}

#[derive(ProtocolRead, ProtocolWrite, Debug, PartialEq, Eq)]
pub struct Prepended {
    #[protocol(tag(type = "u32", value = "self.data.len() as u32"))]
    pub data: Vec<u32>,
}

#[test]
fn can_read_length_prefix_3_elements() {
    assert_eq!(
        WithElementsLength {
            count: 3,
            foo: true,
            data: vec![1, 2, 3],
        },
        WithElementsLength::from_bytes(
            &[
                0, 0, 0, 3, // disjoint length prefix
                1, // boolean true
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            ByteOrder::BigEndian,
        )
        .unwrap()
    );
}

#[test]
fn can_write_auto_length_prefix_3_elements_enum() {
    assert_eq!(
        WithElementsLengthAuto {
            count: 0,
            foo: true,
            data: vec![1, 2, 3],
        }
        .bytes(ByteOrder::BigEndian)
        .unwrap(),
        vec![
            0, 0, 0, 3, // disjoint length prefix
            1, // boolean true
            0, 0, 0, 1, // 1
            0, 0, 0, 2, // 2
            0, 0, 0, 3 // 3
        ],
    );
}

#[test]
fn can_read_length_prefix_3_elements_enum() {
    assert_eq!(
        WithElementsLengthAutoEnum::Variant {
            count: 3,
            foo: true,
            data: vec![1, 2, 3],
        },
        WithElementsLengthAutoEnum::from_bytes(
            &[
                1, // Discriminant
                0, 0, 0, 3, // disjoint length prefix
                1, // boolean true
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            ByteOrder::BigEndian,
        )
        .unwrap()
    );
}

#[test]
fn can_write_auto_length_prefix_3_elements() {
    assert_eq!(
        WithElementsLengthAutoEnum::Variant {
            count: 0,
            foo: true,
            data: vec![1, 2, 3],
        }
        .bytes(ByteOrder::BigEndian)
        .unwrap(),
        vec![
            1, // Discriminant
            0, 0, 0, 3, // disjoint length prefix
            1, // boolean true
            0, 0, 0, 1, // 1
            0, 0, 0, 2, // 2
            0, 0, 0, 3 // 3
        ],
    );
}

#[test]
fn can_read_prepended_length_prefix_3_elements() {
    assert_eq!(
        Prepended {
            data: vec![1, 2, 3],
        },
        Prepended::from_bytes(
            &[
                0, 0, 0, 3, // length prefix
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            ByteOrder::BigEndian,
        )
        .unwrap()
    );
}

#[test]
fn can_write_prepended_length_prefix_3_elements() {
    assert_eq!(
        Prepended {
            data: vec![1, 2, 3],
        }
        .bytes(ByteOrder::BigEndian)
        .unwrap(),
        vec![
            0, 0, 0, 3, // disjoint length prefix
            0, 0, 0, 1, // 1
            0, 0, 0, 2, // 2
            0, 0, 0, 3 // 3
        ],
    );
}
