use std::fmt::Display;

use crate::parser::{
    class::ClassFile,
    constant_pool::{FieldRef, Index},
    field::Field,
    method::Method,
};

#[derive(Debug, Clone)]
pub struct Class {
    class_file: ClassFile,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_file)
    }
}

impl Class {
    pub fn new(class_file: ClassFile) -> Self {
        Self { class_file }
    }

    pub fn package(&self) -> String {
        self.class_file.package.to_string()
    }

    pub fn name(&self) -> String {
        self.class_file.name.to_string()
    }

    pub fn get_main_method(&self) -> Method {
        self.class_file.get_main_method()
    }

    pub fn field_ref(&self, index: &Index) -> Option<FieldRef> {
        self.class_file.field_ref(index)
    }

    pub fn lookup_field(&self, field_ref: &FieldRef) -> Option<Field> {
        self.class_file.get_field(&field_ref.name_and_type)
    }

    pub fn clinit_method(&self) -> Option<Method> {
        self.class_file.clinit_method()
    }
}
