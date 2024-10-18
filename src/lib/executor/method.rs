use crate::parser::{
    self,
    constant_pool::ConstantPool,
    descriptor::{FieldType, MethodDescriptor, ReturnDescriptor},
    method::MethodFlag,
};

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub descriptor: MethodDescriptor,
    access_flags: Vec<MethodFlag>,
}

impl Method {
    pub fn methods(parser_methods: Vec<parser::method::Method>, cp: &ConstantPool) -> Vec<Method> {
        let mut methods = Vec::new();

        for method in &parser_methods {
            methods.push(Method {
                access_flags: method.access_flags.clone(),
                descriptor: MethodDescriptor::new(&cp.utf8(&method.descriptor_index).unwrap()),
                name: cp.utf8(&method.name_index).unwrap(),
            })
        }

        methods
    }

    pub fn is_native(&self) -> bool {
        for flag in &self.access_flags {
            if matches!(flag, MethodFlag::Native) {
                return true;
            }
        }

        false
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
}
