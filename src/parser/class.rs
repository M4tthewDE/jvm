use std::{
    io::{Cursor, Read},
    path::Path,
};

use tracing::{info, instrument};

use crate::parser::parse_u16;

use super::{attribute::Attribute, constant_pool::ConstantPoolInfo, method::Method};

#[derive(Clone, Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: Vec<AccessFlag>,
    pub this_class: u16,
    pub super_class: u16,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    #[instrument]
    pub fn new(p: &Path) -> ClassFile {
        let bytes = std::fs::read(p).unwrap();
        let mut c = Cursor::new(&bytes);

        let mut magic = [0u8; 4];
        c.read_exact(&mut magic).unwrap();
        assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

        let minor_version = parse_u16(&mut c);
        info!(minor_version);

        let major_version = parse_u16(&mut c);
        info!(major_version);

        let constant_pool_count = parse_u16(&mut c);
        info!(constant_pool_count);
        assert!(constant_pool_count > 0);

        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);
        constant_pool.push(ConstantPoolInfo::Reserved);
        for i in 0..constant_pool_count - 1 {
            let cp_info = ConstantPoolInfo::new(&mut c);
            info!("Constant pool info {}: {cp_info:?}", i + 1);
            constant_pool.push(cp_info);
        }

        let access_flags = AccessFlag::flags(parse_u16(&mut c));
        info!("access_flags: {:?}", access_flags);

        let this_class = parse_u16(&mut c);
        info!(this_class);

        let super_class = parse_u16(&mut c);
        info!(super_class);

        let interfaces_count = parse_u16(&mut c);
        info!(interfaces_count);
        assert_eq!(interfaces_count, 0, "not implemented");

        let fields_count = parse_u16(&mut c);
        info!(fields_count);
        assert_eq!(fields_count, 0, "not implemented");

        let methods_count = parse_u16(&mut c);
        info!(methods_count);

        let mut methods = Vec::new();
        for i in 0..methods_count {
            let method = Method::new(&mut c, &constant_pool);
            info!("Method {i}: {method:?}");
            methods.push(method);
        }

        let attributes_count = parse_u16(&mut c);

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

#[cfg(test)]
mod tests {
    use std::{iter::zip, path::PathBuf};

    use crate::parser::{
        attribute::{Attribute, LineNumberTableEntry},
        class::AccessFlag,
        constant_pool::ConstantPoolInfo,
        method::{Method, MethodFlag},
    };

    use super::ClassFile;

    #[test]
    fn test_main() {
        let class = ClassFile::new(&PathBuf::from("testdata/Main.class"));

        assert_eq!(class.minor_version, 0);
        assert_eq!(class.major_version, 61);

        let pool = vec![
            ConstantPoolInfo::Reserved,
            ConstantPoolInfo::MethodRef {
                class_index: 2,
                name_and_type_index: 3,
            },
            ConstantPoolInfo::Class { name_index: 4 },
            ConstantPoolInfo::NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            ConstantPoolInfo::Utf {
                value: "java/lang/Object".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "<init>".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "()V".to_string(),
            },
            ConstantPoolInfo::FieldRef {
                class_index: 8,
                name_and_type_index: 9,
            },
            ConstantPoolInfo::Class { name_index: 10 },
            ConstantPoolInfo::NameAndType {
                name_index: 11,
                descriptor_index: 12,
            },
            ConstantPoolInfo::Utf {
                value: "java/lang/System".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "out".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "Ljava/io/PrintStream;".to_string(),
            },
            ConstantPoolInfo::String { string_index: 14 },
            ConstantPoolInfo::Utf {
                value: "Hello world.".to_string(),
            },
            ConstantPoolInfo::MethodRef {
                class_index: 16,
                name_and_type_index: 17,
            },
            ConstantPoolInfo::Class { name_index: 18 },
            ConstantPoolInfo::NameAndType {
                name_index: 19,
                descriptor_index: 20,
            },
            ConstantPoolInfo::Utf {
                value: "java/io/PrintStream".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "println".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "(Ljava/lang/String;)V".to_string(),
            },
            ConstantPoolInfo::Class { name_index: 22 },
            ConstantPoolInfo::Utf {
                value: "Main".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "Code".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "LineNumberTable".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "main".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "([Ljava/lang/String;)V".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "SourceFile".to_string(),
            },
            ConstantPoolInfo::Utf {
                value: "Main.java".to_string(),
            },
        ];

        assert_eq!(class.constant_pool.len(), pool.len());

        for (i, (testing, good)) in zip(class.constant_pool, pool).enumerate() {
            assert_eq!(testing, good, "mismatch in flag {i}");
        }

        let access_flags = vec![AccessFlag::Public, AccessFlag::Super];
        assert_eq!(class.access_flags.len(), access_flags.len());

        for (i, (testing, good)) in zip(class.access_flags, access_flags).enumerate() {
            assert_eq!(testing, good, "mismatch in flag {i}");
        }

        assert_eq!(class.this_class, 21);
        assert_eq!(class.super_class, 2);

        let methods = vec![
            Method {
                access_flags: vec![MethodFlag::Public],
                name_index: 5,
                descriptor_index: 6,
                attributes: vec![Attribute::Code {
                    max_stacks: 1,
                    max_locals: 1,
                    code: vec![0x2a, 0xb7, 0x00, 0x01, 0xb1],
                    exception_table_length: 0,
                    attributes: vec![Attribute::LineNumberTable {
                        table: vec![LineNumberTableEntry {
                            start_pc: 0,
                            line_number: 1,
                        }],
                    }],
                }],
            },
            Method {
                access_flags: vec![MethodFlag::Public, MethodFlag::Static],
                name_index: 25,
                descriptor_index: 26,
                attributes: vec![Attribute::Code {
                    max_stacks: 2,
                    max_locals: 1,
                    code: vec![0xb2, 0x00, 0x07, 0x12, 0x0d, 0xb6, 0x00, 0x0f, 0xb1],
                    exception_table_length: 0,
                    attributes: vec![Attribute::LineNumberTable {
                        table: vec![
                            LineNumberTableEntry {
                                start_pc: 0,
                                line_number: 3,
                            },
                            LineNumberTableEntry {
                                start_pc: 8,
                                line_number: 4,
                            },
                        ],
                    }],
                }],
            },
        ];

        assert_eq!(class.methods.len(), methods.len());

        for (i, (testing, good)) in zip(class.methods, methods).enumerate() {
            assert_eq!(testing, good, "mismatch in method {i}");
        }

        let attributes = vec![Attribute::SourceFile {
            source_file_index: 28,
        }];

        assert_eq!(class.attributes.len(), attributes.len());

        for (i, (testing, good)) in zip(class.attributes, attributes).enumerate() {
            assert_eq!(testing, good, "mismatch in attribute {i}");
        }
    }
}
