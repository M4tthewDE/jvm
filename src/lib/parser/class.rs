use std::{
    fmt::Display,
    io::{Cursor, Read},
};

use tracing::instrument;

use crate::parser::parse_u16;

use super::{
    attribute::Attribute,
    constant_pool::{ConstantPool, FieldRef, Index, MethodRef, NameAndType},
    field::Field,
    method::Method,
};

#[derive(Clone, Debug)]
pub struct ClassFile {
    pub package: String,
    pub name: String,
    constant_pool: ConstantPool,
    methods: Vec<Method>,
    fields: Vec<Field>,
    access_flags: Vec<AccessFlag>,
}

impl ClassFile {
    #[instrument(skip_all)]
    pub fn new(data: &Vec<u8>, package: String, name: String) -> ClassFile {
        let mut c = Cursor::new(data);

        let mut magic = [0u8; 4];
        c.read_exact(&mut magic).unwrap();
        assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

        let _minor_version = parse_u16(&mut c);
        let _major_version = parse_u16(&mut c);

        let constant_pool_count = parse_u16(&mut c) as usize;
        assert!(constant_pool_count > 0);

        let constant_pool = ConstantPool::new(&mut c, constant_pool_count);
        let access_flags = AccessFlag::flags(parse_u16(&mut c));
        let _this_class = parse_u16(&mut c);
        let _super_class = parse_u16(&mut c);
        let interfaces_count = parse_u16(&mut c);
        assert_eq!(interfaces_count, 0, "not implemented");

        let fields_count = parse_u16(&mut c);
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            let field = Field::new(&mut c, &constant_pool);
            fields.push(field);
        }

        let methods_count = parse_u16(&mut c);

        let mut methods = Vec::new();
        for _ in 0..methods_count {
            let method = Method::new(&mut c, &constant_pool);
            methods.push(method);
        }

        let _attributes = Attribute::attributes(&mut c, &constant_pool);

        ClassFile {
            package,
            name,
            constant_pool,
            methods,
            fields,
            access_flags,
        }
    }

    pub fn get_main_method(&self) -> Method {
        for method in &self.methods {
            if method.is_main(&self.constant_pool) {
                return method.clone();
            }
        }

        panic!("No main method found")
    }

    pub fn is_public(&self) -> bool {
        self.access_flags.contains(&AccessFlag::Public)
    }

    pub fn get_field(&self, name_and_type: &NameAndType) -> Option<Field> {
        for field in &self.fields {
            if field.name(&self.constant_pool) == name_and_type.name
                && field.descriptor(&self.constant_pool) == name_and_type.descriptor
            {
                return Some(field.clone());
            }
        }

        None
    }

    pub fn method(&self, name_and_type: &NameAndType) -> Option<Method> {
        for method in &self.methods {
            if method.descriptor(&self.constant_pool) == name_and_type.descriptor
                && method.name(&self.constant_pool) == name_and_type.name
            {
                return Some(method.clone());
            }
        }

        None
    }

    pub fn has_main(&self) -> bool {
        for method in &self.methods {
            if method.is_main(&self.constant_pool) {
                return true;
            }
        }

        false
    }

    pub fn field_ref(&self, index: &Index) -> Option<FieldRef> {
        self.constant_pool.field_ref(index)
    }

    pub fn clinit_method(&self) -> Option<Method> {
        for method in &self.methods {
            if method.is_clinit(&self.constant_pool) {
                return Some(method.clone());
            }
        }

        None
    }

    pub fn method_ref(&self, method_index: &Index) -> Option<MethodRef> {
        self.constant_pool.method_ref(method_index)
    }
}

impl Display for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.package, self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum AccessFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
}

impl AccessFlag {
    fn flags(val: u16) -> Vec<AccessFlag> {
        let mut flags = Vec::new();

        if (val & 0x0001) != 0 {
            flags.push(AccessFlag::Public);
        }

        if (val & 0x0010) != 0 {
            flags.push(AccessFlag::Final);
        }

        if (val & 0x0020) != 0 {
            flags.push(AccessFlag::Super);
        }

        if (val & 0x0200) != 0 {
            flags.push(AccessFlag::Interface);
        }

        if (val & 0x0400) != 0 {
            flags.push(AccessFlag::Abstract);
        }

        flags
    }
}
