use std::fmt::Display;

use crate::{
    parser::{
        class::ClassFile,
        constant_pool::{FieldRef, Index, MethodRef},
        field::Field,
        method::Method,
    },
    ClassIdentifier, Package,
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

    pub fn package(&self) -> Package {
        self.class_file.class_identifier.package.clone()
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

    pub fn method_ref(&self, method_index: &Index) -> Option<MethodRef> {
        self.class_file.method_ref(method_index)
    }

    pub fn is_native(&self, method_ref: &MethodRef) -> bool {
        self.class_file
            .method(&method_ref.name_and_type)
            .unwrap()
            .is_native()
    }

    pub fn identifier(&self) -> ClassIdentifier {
        self.class_file.class_identifier.clone()
    }
}
