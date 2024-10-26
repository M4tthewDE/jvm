use std::io::Cursor;

use super::{
    attribute::Attribute,
    constant_pool::{ConstantPool, Index},
    parse_u16,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub access_flags: Vec<MethodFlag>,
    pub name_index: Index,
    pub descriptor_index: Index,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Method {
        Method {
            access_flags: MethodFlag::flags(parse_u16(c)),
            name_index: Index::new(parse_u16(c)),
            descriptor_index: Index::new(parse_u16(c)),
            attributes: Attribute::attributes(c, constant_pool),
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
