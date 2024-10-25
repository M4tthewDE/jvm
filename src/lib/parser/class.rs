use std::{
    fmt::Display,
    io::{Cursor, Read},
};

use crate::{parser::parse_u16, ClassIdentifier};

use super::{
    attribute::Attribute,
    constant_pool::{ConstantPool, Index},
    field::Field,
    method::Method,
};

#[derive(Clone, Debug)]
pub struct ClassFile {
    pub class_identifier: ClassIdentifier,
    pub constant_pool: ConstantPool,
    pub _interfaces: Vec<Index>,
    pub methods: Vec<Method>,
    pub fields: Vec<Field>,
    pub access_flags: Vec<AccessFlag>,
}

impl ClassFile {
    pub fn new(data: &Vec<u8>, class_identifier: ClassIdentifier) -> ClassFile {
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

        let mut interfaces = Vec::new();
        for _ in 0..interfaces_count {
            let index = parse_u16(&mut c);
            interfaces.push(Index::new(index));
        }

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
            class_identifier,
            constant_pool,
            _interfaces: interfaces,
            methods,
            fields,
            access_flags,
        }
    }
}

impl Display for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_identifier)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessFlag {
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
