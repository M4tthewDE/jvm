use std::io::{Cursor, Read};

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
        let mut tag = [0u8; 1];
        c.read_exact(&mut tag).unwrap();
        match tag[0] {
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
        let mut name_index = [0u8; 2];
        c.read_exact(&mut name_index).unwrap();
        let name_index = u16::from_be_bytes(name_index);

        ConstantPoolInfo::Class { name_index }
    }

    fn method_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut class_index = [0u8; 2];
        c.read_exact(&mut class_index).unwrap();
        let class_index = u16::from_be_bytes(class_index);

        let mut name_and_type_index = [0u8; 2];
        c.read_exact(&mut name_and_type_index).unwrap();
        let name_and_type_index = u16::from_be_bytes(name_and_type_index);

        ConstantPoolInfo::MethodRef {
            class_index,
            name_and_type_index,
        }
    }

    fn field_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut class_index = [0u8; 2];
        c.read_exact(&mut class_index).unwrap();
        let class_index = u16::from_be_bytes(class_index);

        let mut name_and_type_index = [0u8; 2];
        c.read_exact(&mut name_and_type_index).unwrap();
        let name_and_type_index = u16::from_be_bytes(name_and_type_index);

        ConstantPoolInfo::FieldRef {
            class_index,
            name_and_type_index,
        }
    }

    fn string(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut string_index = [0u8; 2];
        c.read_exact(&mut string_index).unwrap();
        let string_index = u16::from_be_bytes(string_index);

        ConstantPoolInfo::String { string_index }
    }

    fn name_and_type(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut name_index = [0u8; 2];
        c.read_exact(&mut name_index).unwrap();
        let name_index = u16::from_be_bytes(name_index);

        let mut descriptor_index = [0u8; 2];
        c.read_exact(&mut descriptor_index).unwrap();
        let descriptor_index = u16::from_be_bytes(descriptor_index);

        ConstantPoolInfo::NameAndType {
            name_index,
            descriptor_index,
        }
    }

    fn utf8(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut length = [0u8; 2];
        c.read_exact(&mut length).unwrap();
        let length = u16::from_be_bytes(length);

        let mut value = vec![0u8; length as usize];
        c.read_exact(&mut value).unwrap();
        let value = String::from_utf8(value).unwrap();

        ConstantPoolInfo::Utf { value }
    }
}
