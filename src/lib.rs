//! Binary serialization and deserialization compatible with [`serde`].

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod decode;
mod encode;
mod error;
mod read;
mod util;
mod write;

use crate::decode::Decoder;
use crate::encode::Encoder;
pub use crate::error::{Error, Result, ValueType};
use crate::read::{BytesReader, Read};
use crate::write::{BytesWriter, Write};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Serializes a value to binary.
pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = BytesWriter::new();
    let mut encoder = Encoder::new(&mut writer);
    value.serialize(&mut encoder)?;
    Ok(writer.into_inner())
}

/// Serializes a value to binary and writes it to the given writer.
pub fn serialize_into<T, W>(value: &T, writer: &mut W) -> Result<()>
where
    T: Serialize,
    W: Write,
{
    let mut encoder = Encoder::new(writer);
    value.serialize(&mut encoder)?;
    Ok(())
}

/// Deserializes binary data into a new instance of `T`.
pub fn deserialize<'de, 'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'de>,
    'a: 'de,
{
    let mut reader = BytesReader::new(bytes);
    let mut decoder = Decoder::new(&mut reader);
    T::deserialize(&mut decoder)
}

/// Deserializes binary data from the given reader into a new instance of `T`.
pub fn deserialize_from<'de, T, R>(reader: &mut R) -> Result<T>
where
    T: DeserializeOwned,
    R: Read<'de>,
{
    let mut decoder = Decoder::new(reader);
    T::deserialize(&mut decoder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::io::Seek;

    #[test]
    fn test() {
        macro_rules! map {
            ( $( $key:expr => $value:expr ),* $(,)? ) => {{
                #[allow(unused_mut)]
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )*
                m
            }};
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        enum MyEnum {
            UnitVariant,
            NewtypeVariant(u8),
            TupleVariant((), bool, u8),
            StructVariant { a: (), b: bool, c: u8 },
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct MyUnitStruct;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct MyNewtypeStruct(u8);

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct MyTupleStruct((), bool, u8);

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct MyInnerStruct {
            a: (),
            b: bool,
            c: u8,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct MyStruct<'a> {
            bool_field: bool,
            i8_field: i8,
            i16_field: i16,
            i32_field: i32,
            i64_field: i64,
            i128_field: i128,
            u8_field: u8,
            u16_field: u16,
            u32_field: u32,
            u64_field: u64,
            u128_field: u128,
            f32_field: f32,
            f64_field: f64,
            char_field: char,
            str_field: &'a str,
            string_field: String,
            bytes_field: &'a [u8],
            option_none_field: Option<u8>,
            option_some_field: Option<u8>,
            unit_field: (),
            unit_struct_field: MyUnitStruct,
            unit_variant_field: MyEnum,
            newtype_struct_field: MyNewtypeStruct,
            newtype_variant_field: MyEnum,
            seq_field: Vec<u8>,
            tuple_field: ((), bool, u8),
            tuple_struct_field: MyTupleStruct,
            tuple_variant_field: MyEnum,
            map_field: HashMap<u8, u8>,
            struct_field: MyInnerStruct,
            struct_variant_field: MyEnum,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct MyStructNoBorrows {
            bool_field: bool,
            i8_field: i8,
            i16_field: i16,
            i32_field: i32,
            i64_field: i64,
            i128_field: i128,
            u8_field: u8,
            u16_field: u16,
            u32_field: u32,
            u64_field: u64,
            u128_field: u128,
            f32_field: f32,
            f64_field: f64,
            char_field: char,
            string_field: String,
            bytes_field: [u8; 4],
            option_none_field: Option<u8>,
            option_some_field: Option<u8>,
            unit_field: (),
            unit_struct_field: MyUnitStruct,
            unit_variant_field: MyEnum,
            newtype_struct_field: MyNewtypeStruct,
            newtype_variant_field: MyEnum,
            seq_field: Vec<u8>,
            tuple_field: ((), bool, u8),
            tuple_struct_field: MyTupleStruct,
            tuple_variant_field: MyEnum,
            map_field: HashMap<u8, u8>,
            struct_field: MyInnerStruct,
            struct_variant_field: MyEnum,
        }

        let value = MyStruct {
            bool_field: true,
            i8_field: -128,
            i16_field: -32768,
            i32_field: -2147483648,
            i64_field: -9223372036854775808,
            i128_field: -170141183460469231731687303715884105728,
            u8_field: 255,
            u16_field: 65535,
            u32_field: 4294967295,
            u64_field: 18446744073709551615,
            u128_field: 340282366920938463463374607431768211455,
            f32_field: 6.25,
            f64_field: 3.125,
            char_field: 'A',
            str_field: "my string",
            string_field: "my owned string".to_owned(),
            bytes_field: &[0, 1, 2, 3],
            option_none_field: None,
            option_some_field: Some(4),
            unit_field: (),
            unit_struct_field: MyUnitStruct,
            unit_variant_field: MyEnum::UnitVariant,
            newtype_struct_field: MyNewtypeStruct(5),
            newtype_variant_field: MyEnum::NewtypeVariant(6),
            seq_field: vec![7, 8, 9, 10, 11],
            tuple_field: ((), false, 12),
            tuple_struct_field: MyTupleStruct((), true, 13),
            tuple_variant_field: MyEnum::TupleVariant((), false, 14),
            map_field: map! {
                15 => 16,
                17 => 18,
                19 => 20,
            },
            struct_field: MyInnerStruct {
                a: (),
                b: true,
                c: 21,
            },
            struct_variant_field: MyEnum::StructVariant {
                a: (),
                b: false,
                c: 22,
            },
        };

        let value_no_borrows = MyStructNoBorrows {
            bool_field: true,
            i8_field: -128,
            i16_field: -32768,
            i32_field: -2147483648,
            i64_field: -9223372036854775808,
            i128_field: -170141183460469231731687303715884105728,
            u8_field: 255,
            u16_field: 65535,
            u32_field: 4294967295,
            u64_field: 18446744073709551615,
            u128_field: 340282366920938463463374607431768211455,
            f32_field: 6.25,
            f64_field: 3.125,
            char_field: 'A',
            string_field: "my owned string".to_owned(),
            bytes_field: [0, 1, 2, 3],
            option_none_field: None,
            option_some_field: Some(4),
            unit_field: (),
            unit_struct_field: MyUnitStruct,
            unit_variant_field: MyEnum::UnitVariant,
            newtype_struct_field: MyNewtypeStruct(5),
            newtype_variant_field: MyEnum::NewtypeVariant(6),
            seq_field: vec![7, 8, 9, 10, 11],
            tuple_field: ((), false, 12),
            tuple_struct_field: MyTupleStruct((), true, 13),
            tuple_variant_field: MyEnum::TupleVariant((), false, 14),
            map_field: map! {
                15 => 16,
                17 => 18,
                19 => 20,
            },
            struct_field: MyInnerStruct {
                a: (),
                b: true,
                c: 21,
            },
            struct_variant_field: MyEnum::StructVariant {
                a: (),
                b: false,
                c: 22,
            },
        };

        let serialized_value = serialize(&value).unwrap();
        let mut serialized_iter = serialized_value.iter();

        let mut next_n = |n: usize| {
            (0..n).fold(Vec::new(), |mut items, _| {
                if let Some(item) = serialized_iter.next() {
                    items.push(*item);
                }

                items
            })
        };

        // bool_field
        assert_eq!(next_n(1), [1]);

        // i8_field
        assert_eq!(next_n(1), [128]);

        // i16_field
        assert_eq!(next_n(2), [128, 0]);

        // i32_field
        assert_eq!(next_n(4), [128, 0, 0, 0]);

        // i64_field
        assert_eq!(next_n(8), [128, 0, 0, 0, 0, 0, 0, 0]);

        // i128_field
        assert_eq!(
            next_n(16),
            [128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        // u8_field
        assert_eq!(next_n(1), [255]);

        // u16_field
        assert_eq!(next_n(2), [255, 255]);

        // u32_field
        assert_eq!(next_n(4), [255, 255, 255, 255]);

        // u64_field
        assert_eq!(next_n(8), [255, 255, 255, 255, 255, 255, 255, 255]);

        // u128_field
        assert_eq!(
            next_n(16),
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );

        // f32_field
        assert_eq!(next_n(4), [64, 200, 0, 0]);

        // f64_field
        assert_eq!(next_n(8), [64, 9, 0, 0, 0, 0, 0, 0]);

        // char_field
        assert_eq!(next_n(2), [1, 65]);

        // str_field
        assert_eq!(
            next_n(11),
            [1, 9, 109, 121, 32, 115, 116, 114, 105, 110, 103]
        );

        // string_field
        assert_eq!(
            next_n(17),
            [1, 15, 109, 121, 32, 111, 119, 110, 101, 100, 32, 115, 116, 114, 105, 110, 103]
        );

        // bytes_field
        assert_eq!(next_n(6), [1, 4, 0, 1, 2, 3]);

        // option_none_field
        assert_eq!(next_n(1), [0]);

        // option_some_field
        assert_eq!(next_n(2), [1, 4]);

        // unit_field
        // assert_eq!(next_n(0), []);

        // unit_struct_field
        // assert_eq!(next_n(0), []);

        // unit_variant_field
        assert_eq!(next_n(1), [0]);

        // newtype_struct_field
        assert_eq!(next_n(1), [5]);

        // newtype_variant_field
        assert_eq!(next_n(2), [1, 6]);

        // seq_field
        assert_eq!(next_n(7), [1, 5, 7, 8, 9, 10, 11]);

        // tuple_field
        assert_eq!(next_n(2), [0, 12]);

        // tuple_struct_field
        assert_eq!(next_n(2), [1, 13]);

        // tuple_variant_field
        assert_eq!(next_n(3), [2, 0, 14]);

        // map_field
        assert_eq!(next_n(2), [1, 3]);
        let map_pairs = map! {
            next_n(1)[0] => next_n(1)[0],
            next_n(1)[0] => next_n(1)[0],
            next_n(1)[0] => next_n(1)[0],
        };
        let map_expected_pairs = map! {
            15 => 16,
            17 => 18,
            19 => 20,
        };
        assert_eq!(map_pairs, map_expected_pairs);

        // struct_field
        assert_eq!(next_n(2), [1, 21]);

        // struct_variant_field
        assert_eq!(next_n(3), [3, 0, 22]);

        // check that this is the end of the output
        assert!(serialized_iter.next().is_none());

        // deserialize
        let deserialized_value = deserialize::<MyStruct>(&serialized_value).unwrap();
        assert_eq!(value, deserialized_value);
        // dbg!(&deserialized_value);

        // test borrows with file
        let mut file = tempfile::tempfile().unwrap();
        let mut encoder = Encoder::new(&mut file);
        value.serialize(&mut encoder).unwrap();
        file.rewind().unwrap();
        let mut decoder = Decoder::new(&mut file);
        let res = MyStruct::deserialize(&mut decoder);
        assert!(matches!(
            res,
            Result::Err(Error::Custom(message)) if message.as_str() == "invalid type: string \"my string\", expected a borrowed string"
        ));

        // test no borrows with file
        let mut file = tempfile::tempfile().unwrap();
        let mut encoder = Encoder::new(&mut file);
        value_no_borrows.serialize(&mut encoder).unwrap();
        file.rewind().unwrap();
        let mut decoder = Decoder::new(&mut file);
        let deserialized_value_no_borrows = MyStructNoBorrows::deserialize(&mut decoder).unwrap();
        assert_eq!(value_no_borrows, deserialized_value_no_borrows);
        // dbg!(&deserialized_value_no_borrows);

        // check sizes
        let serde_json_serialized_value = serde_json::to_vec(&value).unwrap();
        let bincode_serialized_value = bincode::serialize(&value).unwrap();
        println!(
            "`serde_json` encoded size: {}",
            serde_json_serialized_value.len()
        );
        println!(
            "`bincode` encoded size:    {}",
            bincode_serialized_value.len()
        );
        println!("this encoded size:         {}", serialized_value.len());

        fn assert_send<T: Send>(_x: &T) {}
        fn assert_sync<T: Sync>(_x: &T) {}

        assert_send(&Encoder::new(&mut BytesWriter::new()));
        assert_sync(&Encoder::new(&mut BytesWriter::new()));
        assert_send(&Decoder::new(&mut BytesReader::new(&[])));
        assert_sync(&Decoder::new(&mut BytesReader::new(&[])));

        assert_send(&Encoder::new(&mut tempfile::tempfile().unwrap()));
        assert_sync(&Encoder::new(&mut tempfile::tempfile().unwrap()));
        assert_send(&Decoder::new(&mut tempfile::tempfile().unwrap()));
        assert_sync(&Decoder::new(&mut tempfile::tempfile().unwrap()));
    }
}
