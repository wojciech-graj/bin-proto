use bin_proto::{Protocol, Settings};

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub reason_length: u8,
}

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct WithElementsLength {
    pub count: u32,
    pub foo: bool,
    #[protocol(length = "count as usize")]
    pub data: Vec<u32>,
}

#[derive(bin_proto::Protocol, Debug, PartialEq, Eq)]
pub struct WithElementsLengthAuto {
    #[protocol(write_value = "self.data.len() as u32")]
    pub count: u32,
    pub foo: bool,
    #[protocol(length = "count as usize")]
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
            &Settings::default()
        )
        .unwrap()
    );
}

#[test]
fn can_write_auto_length_prefix_3_elements() {
    assert_eq!(
        WithElementsLengthAuto {
            count: 0,
            foo: true,
            data: vec![1, 2, 3],
        }
        .bytes(&Settings::default())
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
