use std::io::Cursor;

use crate::ClassIdentifier;

use super::{
    descriptor::{Descriptor, FieldType, MethodDescriptor},
    parse_i32, parse_u16, parse_u32, parse_u8, parse_vec,
};

// TODO: there should only be one function that resolves a constant pool element
// the return value is an enum with all possible types
// the caller can then decide what it does with each possible type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameAndType {
    pub name: String,
    pub descriptor: Descriptor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantPoolItem {
    Reserved,
    ClassInfo {
        identifier: ClassIdentifier,
    },
    FieldRef {
        class_identifier: ClassIdentifier,
        name_and_type: NameAndType,
    },
    MethodRef {
        class_identifier: ClassIdentifier,
        name_and_type: NameAndType,
    },
    InterfaceMethodRef {
        class_identifier: ClassIdentifier,
        name_and_type: NameAndType,
    },
    MethodType {
        descriptor: String,
    },
    String {
        value: String,
    },
    Utf {
        text: String,
    },
    Integer {
        val: i32,
    },
    Long {
        val: i64,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: Index,
        name_and_type: NameAndType,
    },
    MethodHandle {
        reference_kind: u8,
        reference: Box<ConstantPoolItem>,
    },
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

    pub fn resolve(&self, index: &Index) -> Option<ConstantPoolItem> {
        self.get(index).map(|i| self.resolve_cp_item(i))?
    }

    fn resolve_cp_item(&self, cp_item: ConstantPoolInfo) -> Option<ConstantPoolItem> {
        match cp_item {
            ConstantPoolInfo::Reserved => Some(ConstantPoolItem::Reserved),
            ConstantPoolInfo::FieldRef {
                class_index,
                name_and_type_index,
            } => {
                if let ConstantPoolInfo::ClassInfo { name_index } = self.get(&class_index)? {
                    let class_identifier = ClassIdentifier::from_utf8(self.utf8(&name_index)?);
                    let name_and_type = self.name_and_type_field(&name_and_type_index)?;

                    Some(ConstantPoolItem::FieldRef {
                        class_identifier,
                        name_and_type,
                    })
                } else {
                    None
                }
            }
            ConstantPoolInfo::MethodRef {
                class_index,
                name_and_type_index,
            } => {
                if let ConstantPoolInfo::ClassInfo { name_index } = self.get(&class_index)? {
                    let class_identifier = ClassIdentifier::from_utf8(self.utf8(&name_index)?);
                    let name_and_type = self.name_and_type_method(&name_and_type_index)?;

                    Some(ConstantPoolItem::MethodRef {
                        class_identifier,
                        name_and_type,
                    })
                } else {
                    None
                }
            }
            ConstantPoolInfo::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => {
                if let ConstantPoolInfo::ClassInfo { name_index } = self.get(&class_index)? {
                    let class_identifier = ClassIdentifier::from_utf8(self.utf8(&name_index)?);
                    let name_and_type = self.name_and_type_field(&name_and_type_index)?;

                    Some(ConstantPoolItem::InterfaceMethodRef {
                        class_identifier,
                        name_and_type,
                    })
                } else {
                    None
                }
            }
            ConstantPoolInfo::String { string_index } => Some(ConstantPoolItem::String {
                value: self.utf8(&string_index)?,
            }),
            ConstantPoolInfo::ClassInfo { name_index } => Some(ConstantPoolItem::ClassInfo {
                identifier: ClassIdentifier::from_utf8(self.utf8(&name_index)?),
            }),
            ConstantPoolInfo::NameAndType {
                name_index: _,
                descriptor_index: _,
            } => None,
            ConstantPoolInfo::Utf { text } => Some(ConstantPoolItem::Utf { text }),
            ConstantPoolInfo::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => Some(ConstantPoolItem::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type: self.name_and_type_method(&name_and_type_index)?,
            }),
            ConstantPoolInfo::Integer(val) => Some(ConstantPoolItem::Integer { val }),
            ConstantPoolInfo::MethodHandle {
                reference_kind,
                reference_index,
            } => Some(ConstantPoolItem::MethodHandle {
                reference_kind,
                reference: Box::new(self.resolve(&reference_index)?),
            }),
            ConstantPoolInfo::MethodType { descriptor_index } => {
                Some(ConstantPoolItem::MethodType {
                    descriptor: self.utf8(&descriptor_index)?,
                })
            }
            ConstantPoolInfo::Long(val) => Some(ConstantPoolItem::Long { val }),
        }
    }

    pub fn utf8(&self, index: &Index) -> Option<String> {
        if let ConstantPoolInfo::Utf { text } = self.infos.get(index.index)? {
            Some(text.to_string())
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
pub enum ConstantPoolInfo {
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
    ClassInfo {
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
        // TODO: I don't think this should be Index
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
            7 => ConstantPoolInfo::class_info(c),
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

    fn class_info(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        ConstantPoolInfo::ClassInfo {
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
