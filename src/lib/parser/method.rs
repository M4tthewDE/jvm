use std::io::Cursor;

use tracing::instrument;

use super::{
    attribute::Attribute,
    constant_pool::ConstantPool,
    descriptor::{FieldType, MethodDescriptor},
    parse_u16,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub access_flags: Vec<MethodFlag>,
    pub name_index: u16,
    pub descriptor_index: usize,
    pub attributes: Vec<Attribute>,
}

impl Method {
    #[instrument(skip_all, name = "method")]
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Method {
        Method {
            access_flags: MethodFlag::flags(parse_u16(c)),
            name_index: parse_u16(c),
            descriptor_index: parse_u16(c) as usize,
            attributes: Attribute::attributes(c, constant_pool),
        }
    }

    pub fn is_main(&self, cp: &ConstantPool) -> bool {
        self.is_public() && self.is_static() && self.name(cp) == "main" && self.has_main_args(cp)
    }

    fn is_public(&self) -> bool {
        self.access_flags.contains(&MethodFlag::Public)
    }

    fn is_static(&self) -> bool {
        self.access_flags.contains(&MethodFlag::Static)
    }

    fn name(&self, cp: &ConstantPool) -> String {
        cp.utf8(self.name_index as usize).unwrap()
    }

    fn has_main_args(&self, cp: &ConstantPool) -> bool {
        let descriptor = MethodDescriptor::new(&cp.utf8(self.descriptor_index).unwrap());
        let main_parameters = vec![FieldType::Array(Box::new(FieldType::Class(
            "java/lang/String".to_string(),
        )))];

        matches!(
            descriptor.return_descriptor,
            crate::parser::descriptor::ReturnDescriptor::Void
        ) && descriptor.parameters == main_parameters
    }

    pub fn get_code_attribute(&self) -> Option<Attribute> {
        for attribute in &self.attributes {
            if let Attribute::Code { .. } = attribute {
                return Some(attribute.clone());
            }
        }

        None
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
