use std::fmt::Display;

use crate::parser::{
    self,
    constant_pool::ConstantPool,
    descriptor::{Descriptor, FieldType},
};
use anyhow::{Context, Result};

use super::stack::Word;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub descriptor: Descriptor,
    pub value: Word,
}

impl Field {
    pub fn fields(
        parser_fields: Vec<parser::field::Field>,
        cp: &ConstantPool,
    ) -> Result<Vec<Field>> {
        let mut fields = Vec::new();
        for field in parser_fields {
            let field_type = FieldType::new(
                &cp.utf8(&field.descriptor_index)
                    .context(format!("no utf8 found"))?,
            )?;
            fields.push(Field {
                name: cp
                    .utf8(&field.name_index)
                    .context(format!("no utf8 at index {:?}", field.name_index))?,
                descriptor: Descriptor::Field(field_type.clone()),
                value: Word::from_field_type(field_type),
            })
        }

        Ok(fields)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.descriptor)
    }
}
