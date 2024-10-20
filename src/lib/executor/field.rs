use crate::parser::{
    self,
    constant_pool::ConstantPool,
    descriptor::{Descriptor, FieldType},
};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub descriptor: Descriptor,
}

impl Field {
    pub fn fields(parser_fields: Vec<parser::field::Field>, cp: &ConstantPool) -> Vec<Field> {
        let mut fields = Vec::new();
        for field in parser_fields {
            fields.push(Field {
                name: cp.utf8(&field.name_index).unwrap(),
                descriptor: Descriptor::Field(FieldType::new(
                    &cp.utf8(&field.descriptor_index).unwrap(),
                )),
            })
        }

        fields
    }
}
