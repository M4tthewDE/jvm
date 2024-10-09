use std::{
    io::{Cursor, Read},
    path::Path,
};

use tracing::{info, instrument};

use super::{attribute::Attribute, constant_pool::ConstantPoolInfo, method::Method};

#[derive(Clone, Debug)]
pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantPoolInfo>,
    access_flags: Vec<AccessFlag>,
    this_class: u16,
    super_class: u16,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

impl ClassFile {
    #[instrument]
    pub fn new(p: &Path) -> ClassFile {
        let bytes = std::fs::read(p).unwrap();
        let mut c = Cursor::new(&bytes);

        let mut magic = [0u8; 4];
        c.read_exact(&mut magic).unwrap();
        assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

        let mut minor_version = [0u8; 2];
        c.read_exact(&mut minor_version).unwrap();
        let minor_version = u16::from_be_bytes(minor_version);
        info!(minor_version);

        let mut major_version = [0u8; 2];
        c.read_exact(&mut major_version).unwrap();
        let major_version = u16::from_be_bytes(major_version);
        info!(major_version);

        let mut constant_pool_count = [0u8; 2];
        c.read_exact(&mut constant_pool_count).unwrap();
        let constant_pool_count = u16::from_be_bytes(constant_pool_count);
        info!(constant_pool_count);
        assert!(constant_pool_count > 0);

        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);
        constant_pool.push(ConstantPoolInfo::Reserved);
        for i in 0..constant_pool_count - 1 {
            let cp_info = ConstantPoolInfo::new(&mut c);
            info!("Constant pool info {}: {cp_info:?}", i + 1);
            constant_pool.push(cp_info);
        }

        let mut access_flags = [0u8; 2];
        c.read_exact(&mut access_flags).unwrap();
        let access_flags = AccessFlag::flags(u16::from_be_bytes(access_flags));
        info!("access_flags: {:?}", access_flags);

        let mut this_class = [0u8; 2];
        c.read_exact(&mut this_class).unwrap();
        let this_class = u16::from_be_bytes(this_class);
        info!(this_class);

        let mut super_class = [0u8; 2];
        c.read_exact(&mut super_class).unwrap();
        let super_class = u16::from_be_bytes(super_class);
        info!(super_class);

        let mut interfaces_count = [0u8; 2];
        c.read_exact(&mut interfaces_count).unwrap();
        let interfaces_count = u16::from_be_bytes(interfaces_count);
        info!(interfaces_count);
        assert_eq!(interfaces_count, 0, "not implemented");

        let mut fields_count = [0u8; 2];
        c.read_exact(&mut fields_count).unwrap();
        let fields_count = u16::from_be_bytes(fields_count);
        info!(fields_count);
        assert_eq!(fields_count, 0, "not implemented");

        let mut methods_count = [0u8; 2];
        c.read_exact(&mut methods_count).unwrap();
        let methods_count = u16::from_be_bytes(methods_count);
        info!(methods_count);

        let mut methods = Vec::new();
        for i in 0..methods_count {
            let method = Method::new(&mut c, &constant_pool);
            info!("Method {i}: {method:?}");
            methods.push(method);
        }

        let mut attributes_count = [0u8; 2];
        c.read_exact(&mut attributes_count).unwrap();
        let attributes_count = u16::from_be_bytes(attributes_count);

        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(&mut c, &constant_pool));
        }

        info!("Attributes: {attributes:?}");

        ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            methods,
            attributes,
        }
    }
}

#[derive(Clone, Debug)]
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