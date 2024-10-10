use std::io::{Cursor, Read};

use super::{parse_u16, parse_u8};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantPoolInfo {
    Reserved,
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Class {
        name_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf {
        value: String,
    },
}

impl ConstantPoolInfo {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let tag = parse_u8(c);

        match tag {
            1 => ConstantPoolInfo::utf8(c),
            7 => ConstantPoolInfo::class(c),
            8 => ConstantPoolInfo::string(c),
            9 => ConstantPoolInfo::field_ref(c),
            10 => ConstantPoolInfo::method_ref(c),
            12 => ConstantPoolInfo::name_and_type(c),
            t => panic!("invalid constant pool tag {t}"),
        }
    }

    fn class(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::Class {
            name_index: parse_u16(c),
        }
    }

    fn method_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::MethodRef {
            class_index: parse_u16(c),
            name_and_type_index: parse_u16(c),
        }
    }

    fn field_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::FieldRef {
            class_index: parse_u16(c),
            name_and_type_index: parse_u16(c),
        }
    }

    fn string(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::String {
            string_index: parse_u16(c),
        }
    }

    fn name_and_type(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::NameAndType {
            name_index: parse_u16(c),
            descriptor_index: parse_u16(c),
        }
    }

    fn utf8(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let length = parse_u16(c);

        let mut value = vec![0u8; length as usize];
        c.read_exact(&mut value).unwrap();
        let value = String::from_utf8(value).unwrap();

        ConstantPoolInfo::Utf { value }
    }
}
