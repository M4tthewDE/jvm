use crate::parser::{
    self,
    attribute::Attribute,
    constant_pool::ConstantPool,
    descriptor::{FieldType, MethodDescriptor, ReturnDescriptor},
    method::MethodFlag,
};
use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub name: String,
    pub descriptor: MethodDescriptor,
    access_flags: Vec<MethodFlag>,
    attributes: Vec<Attribute>,
}

impl Method {
    pub fn methods(
        parser_methods: Vec<parser::method::Method>,
        cp: &ConstantPool,
    ) -> Result<Vec<Method>> {
        let mut methods = Vec::new();

        for method in &parser_methods {
            methods.push(Method {
                name: cp
                    .utf8(&method.name_index)
                    .context(format!("no utf8 entry at {:?}", method.name_index))?,
                descriptor: MethodDescriptor::new(
                    &cp.utf8(&method.descriptor_index)
                        .context(format!("no utf8 entry at {:?}", method.name_index))?,
                )?,
                access_flags: method.access_flags.clone(),
                attributes: method.attributes.clone(),
            })
        }

        Ok(methods)
    }

    pub fn is_native(&self) -> bool {
        self.access_flags.contains(&MethodFlag::Native)
    }

    pub fn is_clinit(&self) -> bool {
        self.name == "<clinit>" && self.descriptor.return_descriptor == ReturnDescriptor::Void
    }

    fn is_public(&self) -> bool {
        self.access_flags.contains(&MethodFlag::Public)
    }

    fn is_static(&self) -> bool {
        self.access_flags.contains(&MethodFlag::Static)
    }

    pub fn is_varargs(&self) -> bool {
        self.access_flags.contains(&MethodFlag::Varargs)
    }

    fn has_main_args(&self) -> bool {
        let main_parameters = vec![FieldType::Array(Box::new(FieldType::Class(
            "java/lang/String".to_string(),
        )))];

        matches!(self.descriptor.return_descriptor, ReturnDescriptor::Void)
            && self.descriptor.parameters == main_parameters
    }

    pub fn is_main(&self) -> bool {
        self.is_public() && self.is_static() && self.name == "main" && self.has_main_args()
    }

    pub fn code_attribute(&self) -> Result<Attribute> {
        for attribute in &self.attributes {
            if let Attribute::Code { .. } = attribute {
                return Ok(attribute.clone());
            }
        }

        bail!("no code attribute found")
    }
}
