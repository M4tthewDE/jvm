use crate::parser::{constant_pool::Index, parse_u16};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnerClass {
    inner_class_info_index: Index,
    outer_class_info_index: Index,
    inner_name_index: Index,
    inner_class_access_flags: Vec<AccessFlag>,
}

impl InnerClass {
    pub fn new(c: &mut std::io::Cursor<&Vec<u8>>) -> Self {
        Self {
            inner_class_info_index: Index::new(parse_u16(c)),
            outer_class_info_index: Index::new(parse_u16(c)),
            inner_name_index: Index::new(parse_u16(c)),
            inner_class_access_flags: AccessFlag::flags(parse_u16(c)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
}

impl AccessFlag {
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

        if (val & 0x0200) != 0 {
            flags.push(Self::Interface);
        }

        if (val & 0x0400) != 0 {
            flags.push(Self::Abstract);
        }

        if (val & 0x1000) != 0 {
            flags.push(Self::Synthetic);
        }

        if (val & 0x2000) != 0 {
            flags.push(Self::Annotation);
        }

        if (val & 0x4000) != 0 {
            flags.push(Self::Enum);
        }

        flags
    }
}
