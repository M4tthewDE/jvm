use std::io::Cursor;

use super::{attribute::Attribute, constant_pool::ConstantPool, parse_u16};

#[derive(Clone, Debug)]
pub struct Field {
    pub access_flags: Vec<FieldFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Self {
        let access_flags = FieldFlag::flags(parse_u16(c));
        let name_index = parse_u16(c);
        let descriptor_index = parse_u16(c);
        let attributes = Attribute::attributes(c, constant_pool);

        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum,
}

impl FieldFlag {
    fn flags(val: u16) -> Vec<Self> {
        let mut flags = Vec::new();

        if (val & 0x0001) != 0 {
            flags.push(Self::Public);
        }

        if (val & 0x0002) != 0 {
            flags.push(Self::Private);
        }

        if (val & 0x0004) != 0 {
            flags.push(Self::Protected);
        }

        if (val & 0x0008) != 0 {
            flags.push(Self::Static);
        }

        if (val & 0x0010) != 0 {
            flags.push(Self::Final);
        }

        if (val & 0x0040) != 0 {
            flags.push(Self::Volatile);
        }

        if (val & 0x0080) != 0 {
            flags.push(Self::Transient);
        }

        if (val & 0x1000) != 0 {
            flags.push(Self::Synthetic);
        }

        if (val & 0x4000) != 0 {
            flags.push(Self::Enum);
        }

        flags
    }
}
