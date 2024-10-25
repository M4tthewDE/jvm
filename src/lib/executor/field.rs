use std::fmt::Display;

use crate::parser::{
    self,
    constant_pool::ConstantPool,
    descriptor::{Descriptor, FieldType},
};

use super::stack::Word;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub descriptor: Descriptor,
    pub value: Word,
}

impl Field {
    pub fn fields(parser_fields: Vec<parser::field::Field>, cp: &ConstantPool) -> Vec<Field> {
        let mut fields = Vec::new();
        for field in parser_fields {
            let field_type = FieldType::new(&cp.utf8(&field.descriptor_index).unwrap());
            fields.push(Field {
                name: cp.utf8(&field.name_index).unwrap(),
                descriptor: Descriptor::Field(field_type.clone()),
                value: Word::from_field_type(field_type),
            })
        }

        fields
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.descriptor)
    }
}
