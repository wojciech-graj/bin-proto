#![cfg(test)]

macro_rules! verify_read_back {
    ($name:ident => $parcel:expr) => {
        pub mod $name {
            use super::*;
            use bin_proto::{self, Protocol, Settings};

            fn verify_read_back(settings: &Settings) {
                let read_back =
                    Protocol::from_bytes(&$parcel.bytes(&settings).unwrap()[..], &settings)
                        .unwrap();
                assert_eq!($parcel, read_back);
            }

            #[test]
            fn can_read_back_default_settings() {
                verify_read_back(&bin_proto::Settings::default());
            }

            mod byte_order {
                use super::*;
                use bin_proto::{ByteOrder, Settings};

                #[test]
                fn can_read_back_in_big_endian() {
                    verify_read_back(&Settings {
                        byte_order: ByteOrder::BigEndian,
                        ..Settings::default()
                    });
                }

                #[test]
                fn can_read_back_in_little_endian() {
                    verify_read_back(&Settings {
                        byte_order: ByteOrder::LittleEndian,
                        ..Settings::default()
                    });
                }
            }
        }
    };
}

#[cfg(test)]
mod aligned;
#[cfg(test)]
mod enum_trait;
#[cfg(test)]
mod enums;
#[cfg(test)]
mod flexible_array_member;
#[cfg(test)]
mod ipv4;
#[cfg(test)]
mod length_prefix;
#[cfg(test)]
mod structs;
