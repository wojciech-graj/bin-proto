use bin_proto::{ByteOrder, Protocol};

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub reason_length: u8,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct WithElementsLength {
    pub count: u32,
    pub foo: bool,
    #[protocol(length = "count as usize")]
    pub data: Vec<u32>,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
pub struct WithElementsLengthAuto {
    #[protocol(write_value = "self.data.len() as u32")]
    pub count: u32,
    pub foo: bool,
    #[protocol(length = "count as usize")]
    pub data: Vec<u32>,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
#[protocol(discriminant_type = "u8")]
pub enum WithElementsLengthAutoEnum {
    #[protocol(discriminant = "1")]
    Variant {
        #[protocol(write_value = "data.len() as u32")]
        count: u32,
        foo: bool,
        #[protocol(length = "count as usize")]
        data: Vec<u32>,
    },
}

#[test]
fn can_read_length_prefix_3_elements() {
    assert_eq!(
        WithElementsLength {
            count: 3,
            foo: true,
            data: vec![1, 2, 3],
        },
        WithElementsLength::from_bytes_ctx(
            &[
                0, 0, 0, 3, // disjoint length prefix
                1, // boolean true
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            ByteOrder::BigEndian,
            &mut ()
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
        .bytes_ctx(ByteOrder::BigEndian, &mut ())
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
        WithElementsLengthAutoEnum::from_bytes_ctx(
            &[
                1, // Discriminant
                0, 0, 0, 3, // disjoint length prefix
                1, // boolean true
                0, 0, 0, 1, // 1
                0, 0, 0, 2, // 2
                0, 0, 0, 3 // 3
            ],
            ByteOrder::BigEndian,
            &mut ()
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
        .bytes_ctx(ByteOrder::BigEndian, &mut ())
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
