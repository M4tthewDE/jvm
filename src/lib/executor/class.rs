use anyhow::{bail, Result};
use std::fmt::Display;

use crate::{
    parser::{
        class::{AccessFlag, ClassFile},
        constant_pool::{ConstantPool, ConstantPoolItem, Index, NameAndType},
        descriptor::MethodDescriptor,
    },
    ClassIdentifier, Package,
};

use super::{field::Field, method::Method, stack::Word};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub identifier: ClassIdentifier,
    constant_pool: ConstantPool,
    methods: Vec<Method>,
    pub fields: Vec<Field>,
    access_flags: Vec<AccessFlag>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl Class {
    pub fn new(class_file: ClassFile) -> Result<Self> {
        Ok(Self {
            identifier: class_file.class_identifier,
            constant_pool: class_file.constant_pool.clone(),
            fields: Field::fields(class_file.fields, &class_file.constant_pool)?,
            methods: Method::methods(class_file.methods, &class_file.constant_pool)?,
            access_flags: class_file.access_flags.clone(),
        })
    }

    pub fn package(&self) -> Package {
        self.identifier.package.clone()
    }

    pub fn is_public(&self) -> bool {
        self.access_flags.contains(&AccessFlag::Public)
    }

    pub fn main_method(&self) -> Result<Method> {
        for method in &self.methods {
            if method.is_main() {
                return Ok(method.clone());
            }
        }

        bail!("no main method found")
    }

    pub fn field(&self, name_and_type: &NameAndType) -> Result<Field> {
        for field in &self.fields {
            if field.name == name_and_type.name && field.descriptor == name_and_type.descriptor {
                return Ok(field.clone());
            }
        }

        bail!("field not found")
    }

    pub fn clinit_method(&self) -> Option<Method> {
        for method in &self.methods {
            if method.is_clinit() {
                return Some(method.clone());
            }
        }

        None
    }

    pub fn is_native(&self, name: &str, descriptor: &MethodDescriptor) -> Result<bool> {
        Ok(self.method(name, descriptor)?.is_native())
    }

    pub fn has_main(&self) -> bool {
        for method in &self.methods {
            if method.is_main() {
                return true;
            }
        }

        false
    }

    pub fn method(&self, name: &str, descriptor: &MethodDescriptor) -> Result<Method> {
        for method in &self.methods {
            if method.descriptor == *descriptor && method.name == name {
                return Ok(method.clone());
            }
        }

        bail!("method not found")
    }

    pub fn resolve_in_cp(&self, index: &Index) -> Option<ConstantPoolItem> {
        self.constant_pool.resolve(index)
    }

    pub fn set_field(&mut self, field: &Field, value: &Word) {
        for f in &mut self.fields {
            if f == field {
                f.value = value.clone();
                return;
            }
        }

        panic!("field {field} not found in {self}");
    }
}
