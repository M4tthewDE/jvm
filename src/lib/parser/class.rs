use std::io::{Cursor, Read};

use tracing::instrument;

use crate::parser::parse_u16;

use super::{attribute::Attribute, constant_pool::ConstantPool, field::Field, method::Method};

#[derive(Clone, Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    access_flags: Vec<AccessFlag>,
    pub this_class: u16,
    pub super_class: u16,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    #[instrument(skip_all)]
    pub fn new(data: &Vec<u8>) -> ClassFile {
        let mut c = Cursor::new(data);

        let mut magic = [0u8; 4];
        c.read_exact(&mut magic).unwrap();
        assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

        let minor_version = parse_u16(&mut c);
        let major_version = parse_u16(&mut c);

        let constant_pool_count = parse_u16(&mut c) as usize;
        assert!(constant_pool_count > 0);

        let constant_pool = ConstantPool::new(&mut c, constant_pool_count);
        let access_flags = AccessFlag::flags(parse_u16(&mut c));
        let this_class = parse_u16(&mut c);
        let super_class = parse_u16(&mut c);
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

        let attributes = Attribute::attributes(&mut c, &constant_pool);

        ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            fields,
            methods,
            attributes,
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
