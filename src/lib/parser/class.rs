use std::{
    io::{Cursor, Read},
    path::Path,
};

use tracing::instrument;

use crate::parser::parse_u16;

use super::{attribute::Attribute, constant_pool::ConstantPool, method::Method};

#[derive(Clone, Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    access_flags: Vec<AccessFlag>,
    pub this_class: u16,
    pub super_class: u16,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    #[instrument]
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
        assert_eq!(fields_count, 0, "not implemented");

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
            ConstantPoolInfo::ClassRef { name_index: 4 },
            ConstantPoolInfo::NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            ConstantPoolInfo::Utf {
                text: "java/lang/Object".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "<init>".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "()V".to_string(),
            },
            ConstantPoolInfo::FieldRef {
                class_index: 8,
                name_and_type_index: 9,
            },
            ConstantPoolInfo::ClassRef { name_index: 10 },
            ConstantPoolInfo::NameAndType {
                name_index: 11,
                descriptor_index: 12,
            },
            ConstantPoolInfo::Utf {
                text: "java/lang/System".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "out".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "Ljava/io/PrintStream;".to_string(),
            },
            ConstantPoolInfo::String { string_index: 14 },
            ConstantPoolInfo::Utf {
                text: "Hello world.".to_string(),
            },
            ConstantPoolInfo::MethodRef {
                class_index: 16,
                name_and_type_index: 17,
            },
            ConstantPoolInfo::ClassRef { name_index: 18 },
            ConstantPoolInfo::NameAndType {
                name_index: 19,
                descriptor_index: 20,
            },
            ConstantPoolInfo::Utf {
                text: "java/io/PrintStream".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "println".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "(Ljava/lang/String;)V".to_string(),
            },
            ConstantPoolInfo::ClassRef { name_index: 22 },
            ConstantPoolInfo::Utf {
                text: "Main".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "Code".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "LineNumberTable".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "main".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "([Ljava/lang/String;)V".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "SourceFile".to_string(),
            },
            ConstantPoolInfo::Utf {
                text: "Main.java".to_string(),
            },
        ];

        assert_eq!(class.constant_pool.infos.len(), pool.len());

        for (i, (testing, good)) in zip(class.constant_pool.infos, pool).enumerate() {
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
