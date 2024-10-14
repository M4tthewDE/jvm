use std::io::Cursor;

use super::{parse_u16, parse_u8, parse_vec};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameAndType {
    name: String,
    descriptor: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldRef {
    pub class_ref: ClassRef,
    name_and_type: NameAndType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassRef {
    pub package: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantPool {
    // TODO: does this need to be pub?
    pub infos: Vec<ConstantPoolInfo>,
}

impl ConstantPool {
    pub fn new(c: &mut Cursor<&Vec<u8>>, count: usize) -> ConstantPool {
        let mut infos = Vec::with_capacity(count);
        infos.push(ConstantPoolInfo::Reserved);
        for _ in 0..count - 1 {
            let info = ConstantPoolInfo::new(c);
            infos.push(info);
        }

        ConstantPool { infos }
    }

    pub fn utf8(&self, index: usize) -> Option<String> {
        if let ConstantPoolInfo::Utf { text } = self.infos.get(index).unwrap() {
            Some(text.to_string())
        } else {
            None
        }
    }

    pub fn field_ref(&self, index: usize) -> Option<FieldRef> {
        if let ConstantPoolInfo::FieldRef {
            class_index,
            name_and_type_index,
        } = self.infos.get(index).unwrap()
        {
            Some(FieldRef {
                class_ref: self.class_ref(*class_index as usize).unwrap(),
                name_and_type: self.name_and_type(*name_and_type_index as usize).unwrap(),
            })
        } else {
            None
        }
    }

    fn class_ref(&self, index: usize) -> Option<ClassRef> {
        if let ConstantPoolInfo::ClassRef { name_index } = self.infos.get(index).unwrap() {
            let text = self.utf8(*name_index as usize).unwrap();
            let text = text.replace("/", ".");
            let parts: Vec<&str> = text.split(".").collect();
            let name = parts.last().unwrap().to_string();
            let package = parts[..parts.len() - 1].join(".");
            Some(ClassRef { name, package })
        } else {
            None
        }
    }

    fn name_and_type(&self, index: usize) -> Option<NameAndType> {
        if let ConstantPoolInfo::NameAndType {
            name_index,
            descriptor_index,
        } = self.infos.get(index).unwrap()
        {
            Some(NameAndType {
                name: self.utf8(*name_index as usize).unwrap(),
                descriptor: self.utf8(*descriptor_index as usize).unwrap(),
            })
        } else {
            None
        }
    }
}

// TODO: does this need to be pub?
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
    ClassRef {
        name_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf {
        text: String,
    },
}

impl ConstantPoolInfo {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let tag = parse_u8(c);

        match tag {
            1 => ConstantPoolInfo::utf8(c),
            7 => ConstantPoolInfo::class_ref(c),
            8 => ConstantPoolInfo::string(c),
            9 => ConstantPoolInfo::field_ref(c),
            10 => ConstantPoolInfo::method_ref(c),
            12 => ConstantPoolInfo::name_and_type(c),
            t => panic!("invalid constant pool tag {t}"),
        }
    }

    fn class_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::ClassRef {
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
        let length = parse_u16(c) as usize;
        let text = String::from_utf8(parse_vec(c, length)).unwrap();

        ConstantPoolInfo::Utf { text }
    }
}
