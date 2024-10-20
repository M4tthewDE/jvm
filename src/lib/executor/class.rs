use std::fmt::Display;

use crate::{
    parser::{
        class::{AccessFlag, ClassFile},
        constant_pool::{ConstantPool, FieldRef, Index, MethodRef, NameAndType},
        descriptor::MethodDescriptor,
    },
    ClassIdentifier, Package,
};

use super::{field::Field, method::Method};

#[derive(Debug, Clone)]
pub struct Class {
    pub identifier: ClassIdentifier,
    constant_pool: ConstantPool,
    methods: Vec<Method>,
    fields: Vec<Field>,
    access_flags: Vec<AccessFlag>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl Class {
    pub fn new(class_file: ClassFile) -> Self {
        Self {
            identifier: class_file.class_identifier,
            constant_pool: class_file.constant_pool.clone(),
            fields: Field::fields(class_file.fields, &class_file.constant_pool),
            methods: Method::methods(class_file.methods, &class_file.constant_pool),
            access_flags: class_file.access_flags.clone(),
        }
    }

    pub fn package(&self) -> Package {
        self.identifier.package.clone()
    }

    pub fn is_public(&self) -> bool {
        self.access_flags.contains(&AccessFlag::Public)
    }

    pub fn main_method(&self) -> Option<Method> {
        for method in &self.methods {
            if method.is_main() {
                return Some(method.clone());
            }
        }

        None
    }

    pub fn field(&self, name_and_type: &NameAndType) -> Option<Field> {
        for field in &self.fields {
            if field.name == name_and_type.name && field.descriptor == name_and_type.descriptor {
                return Some(field.clone());
            }
        }

        None
    }

    pub fn clinit_method(&self) -> Option<Method> {
        for method in &self.methods {
            if method.is_clinit() {
                return Some(method.clone());
            }
        }

        None
    }

    pub fn is_native(&self, name: &str, descriptor: &MethodDescriptor) -> bool {
        self.method(name, descriptor).unwrap().is_native()
    }

    pub fn has_main(&self) -> bool {
        for method in &self.methods {
            if method.is_main() {
                return true;
            }
        }

        false
    }

    pub fn method(&self, name: &str, descriptor: &MethodDescriptor) -> Option<Method> {
        for method in &self.methods {
            if method.descriptor == *descriptor && method.name == name {
                return Some(method.clone());
            }
        }

        None
    }

    pub fn field_ref(&self, field_ref_index: &Index) -> Option<FieldRef> {
        self.constant_pool.field_ref(field_ref_index)
    }

    pub fn method_ref(&self, method_ref_index: &Index) -> Option<MethodRef> {
        self.constant_pool.method_ref(method_ref_index)
    }
}
