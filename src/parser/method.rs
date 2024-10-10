use std::io::{Cursor, Read};

use super::{attribute::Attribute, constant_pool::ConstantPoolInfo};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub access_flags: Vec<MethodFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Method {
        let mut access_flags = [0u8; 2];
        c.read_exact(&mut access_flags).unwrap();
        let access_flags = MethodFlag::flags(u16::from_be_bytes(access_flags));

        let mut name_index = [0u8; 2];
        c.read_exact(&mut name_index).unwrap();
        let name_index = u16::from_be_bytes(name_index);

        let mut descriptor_index = [0u8; 2];
        c.read_exact(&mut descriptor_index).unwrap();
        let descriptor_index = u16::from_be_bytes(descriptor_index);

        let mut attributes_count = [0u8; 2];
        c.read_exact(&mut attributes_count).unwrap();
        let attributes_count = u16::from_be_bytes(attributes_count);

        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(c, constant_pool));
        }

        Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MethodFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Native,
    Abstract,
}

impl MethodFlag {
    fn flags(val: u16) -> Vec<MethodFlag> {
        let mut flags = Vec::new();

        if (val & 0x0001) != 0 {
            flags.push(MethodFlag::Public);
        }

        if (val & 0x0002) != 0 {
            flags.push(MethodFlag::Private);
        }

        if (val & 0x0004) != 0 {
            flags.push(MethodFlag::Protected);
        }

        if (val & 0x0008) != 0 {
            flags.push(MethodFlag::Static);
        }

        if (val & 0x0010) != 0 {
            flags.push(MethodFlag::Final);
        }

        if (val & 0x0020) != 0 {
            flags.push(MethodFlag::Synchronized);
        }

        if (val & 0x0100) != 0 {
            flags.push(MethodFlag::Native);
        }

        if (val & 0x0400) != 0 {
            flags.push(MethodFlag::Abstract);
        }

        flags
    }
}
