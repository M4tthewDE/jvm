use std::io::Cursor;

use crate::{ClassIdentifier, ClassName, Package};

use super::{
    descriptor::{Descriptor, FieldType, MethodDescriptor},
    parse_i32, parse_u16, parse_u32, parse_u8, parse_vec,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameAndType {
    pub name: String,
    pub descriptor: Descriptor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldRef {
    pub class_ref: ClassRef,
    pub name_and_type: NameAndType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassRef {
    pub class_identifier: ClassIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodRef {
    pub class: ClassRef,
    pub name_and_type: NameAndType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantPool {
    infos: Vec<ConstantPoolInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Index {
    index: usize,
}

impl Index {
    pub fn new<T: Into<usize>>(index: T) -> Self {
        Self {
            index: index.into(),
        }
    }
}

impl ConstantPool {
    pub fn new(c: &mut Cursor<&Vec<u8>>, count: usize) -> ConstantPool {
        let mut infos = vec![ConstantPoolInfo::Reserved; count];

        let mut i = 0;
        loop {
            let info = ConstantPoolInfo::new(c);
            if matches!(info, ConstantPoolInfo::Long(..)) {
                i += 2;
            } else {
                i += 1;
            }

            infos[i] = info;

            if i == count - 1 {
                break;
            }
        }

        ConstantPool { infos }
    }

    fn get(&self, index: &Index) -> Option<ConstantPoolInfo> {
        self.infos.get(index.index).cloned()
    }

    pub fn utf8(&self, index: &Index) -> Option<String> {
        if let ConstantPoolInfo::Utf { text } = self.infos.get(index.index)? {
            Some(text.to_string())
        } else {
            None
        }
    }

    pub fn field_ref(&self, index: &Index) -> Option<FieldRef> {
        if let ConstantPoolInfo::FieldRef {
            class_index,
            name_and_type_index,
        } = self.infos.get(index.index).unwrap()
        {
            Some(FieldRef {
                class_ref: self.class_ref(class_index).unwrap(),
                name_and_type: self.name_and_type_field(name_and_type_index).unwrap(),
            })
        } else {
            None
        }
    }

    pub fn method_ref(&self, index: &Index) -> Option<MethodRef> {
        match self.infos.get(index.index).unwrap() {
            ConstantPoolInfo::MethodRef {
                class_index,
                name_and_type_index,
            } => Some(MethodRef {
                class: self.class_ref(class_index).unwrap(),
                name_and_type: self.name_and_type_method(name_and_type_index).unwrap(),
            }),
            _ => None,
        }
    }

    pub fn class_ref(&self, index: &Index) -> Option<ClassRef> {
        if let ConstantPoolInfo::ClassRef { name_index } = self.get(index).unwrap() {
            let text = self.utf8(&name_index).unwrap();
            let text = text.replace("/", ".");
            let parts: Vec<&str> = text.split(".").collect();
            let name = ClassName::new(parts.last().unwrap().to_string());
            let package = Package::new(parts[..parts.len() - 1].join("."));
            let class_identifier = ClassIdentifier::new(package, name);
            Some(ClassRef { class_identifier })
        } else {
            None
        }
    }

    fn name_and_type_method(&self, index: &Index) -> Option<NameAndType> {
        if let ConstantPoolInfo::NameAndType {
            name_index,
            descriptor_index,
        } = self.get(index).unwrap()
        {
            Some(NameAndType {
                name: self.utf8(&name_index).unwrap(),
                descriptor: Descriptor::Method(MethodDescriptor::new(
                    &self.utf8(&descriptor_index).unwrap(),
                )),
            })
        } else {
            None
        }
    }

    fn name_and_type_field(&self, index: &Index) -> Option<NameAndType> {
        if let ConstantPoolInfo::NameAndType {
            name_index,
            descriptor_index,
        } = self.get(index).unwrap()
        {
            Some(NameAndType {
                name: self.utf8(&name_index).unwrap(),
                descriptor: Descriptor::Field(FieldType::new(
                    &self.utf8(&descriptor_index).unwrap(),
                )),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ConstantPoolInfo {
    Reserved,
    FieldRef {
        class_index: Index,
        name_and_type_index: Index,
    },
    MethodRef {
        class_index: Index,
        name_and_type_index: Index,
    },
    InterfaceMethodRef {
        class_index: Index,
        name_and_type_index: Index,
    },
    String {
        string_index: Index,
    },
    ClassRef {
        name_index: Index,
    },
    NameAndType {
        name_index: Index,
        descriptor_index: Index,
    },
    Utf {
        text: String,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: Index,
        name_and_type_index: Index,
    },
    Integer(i32),
    MethodHandle {
        reference_kind: u8,
        reference_index: Index,
    },
    MethodType {
        descriptor_index: Index,
    },
    Long(i64),
}

impl ConstantPoolInfo {
    fn new(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let tag = parse_u8(c);

        match tag {
            1 => ConstantPoolInfo::utf8(c),
            3 => ConstantPoolInfo::integer(c),
            5 => ConstantPoolInfo::long(c),
            7 => ConstantPoolInfo::class_ref(c),
            8 => ConstantPoolInfo::string(c),
            9 => ConstantPoolInfo::field_ref(c),
            10 => ConstantPoolInfo::method_ref(c),
            11 => ConstantPoolInfo::interface_method_ref(c),
            12 => ConstantPoolInfo::name_and_type(c),
            15 => ConstantPoolInfo::method_handle(c),
            16 => ConstantPoolInfo::method_type(c),
            18 => ConstantPoolInfo::invoke_dynamic(c),
            t => panic!("invalid constant pool tag {t}"),
        }
    }

    fn class_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::ClassRef {
            name_index: Index::new(parse_u16(c)),
        }
    }

    fn method_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::MethodRef {
            class_index: Index::new(parse_u16(c)),
            name_and_type_index: Index::new(parse_u16(c)),
        }
    }

    fn interface_method_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::InterfaceMethodRef {
            class_index: Index::new(parse_u16(c)),
            name_and_type_index: Index::new(parse_u16(c)),
        }
    }

    fn field_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::FieldRef {
            class_index: Index::new(parse_u16(c)),
            name_and_type_index: Index::new(parse_u16(c)),
        }
    }

    fn string(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::String {
            string_index: Index::new(parse_u16(c)),
        }
    }

    fn name_and_type(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::NameAndType {
            name_index: Index::new(parse_u16(c)),
            descriptor_index: Index::new(parse_u16(c)),
        }
    }

    fn utf8(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let length = parse_u16(c) as usize;
        let text = String::from_utf8(parse_vec(c, length)).unwrap();

        ConstantPoolInfo::Utf { text }
    }

    fn invoke_dynamic(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::InvokeDynamic {
            bootstrap_method_attr_index: Index::new(parse_u16(c)),
            name_and_type_index: Index::new(parse_u16(c)),
        }
    }

    fn integer(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::Integer(parse_i32(c))
    }

    fn method_handle(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let reference_kind = parse_u8(c);
        assert!(
            reference_kind > 0 && reference_kind < 10,
            "invalid value for reference_kind {reference_kind}"
        );

        ConstantPoolInfo::MethodHandle {
            reference_kind,
            reference_index: Index::new(parse_u16(c)),
        }
    }

    fn method_type(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::MethodType {
            descriptor_index: Index::new(parse_u16(c)),
        }
    }

    fn long(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let high_bytes = parse_u32(c) as i64;
        let low_bytes = parse_u32(c) as i64;
        ConstantPoolInfo::Long((high_bytes << 8) | low_bytes)
    }
}
