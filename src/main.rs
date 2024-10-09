// https://www.cs.miami.edu/home/burt/reference/java/language_vm_specification.pdf
// https://blogs.oracle.com/javamagazine/post/how-the-jvm-locates-loads-and-runs-libraries
// https://www.mobilefish.com/services/java_decompiler/java_decompiler.php

use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    classpath: Option<Vec<String>>,
    #[arg(short, long)]
    main_class: String,
}

fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();

    let mut class_loader = ClassLoader::new(cli.classpath.unwrap_or_default());
    class_loader.load(&"".to_string(), &cli.main_class);
}

struct ClassLoader {
    classpath: Vec<PathBuf>,
    classes: HashMap<String, ClassFile>,
}

impl ClassLoader {
    fn new(cp: Vec<String>) -> ClassLoader {
        let mut classpath = Vec::new();
        for path in cp {
            let p = PathBuf::from(path);
            if !p.exists() {
                panic!("Invalid path in classpath: {p:?}");
            }

            classpath.push(p);
        }

        ClassLoader {
            classpath,
            classes: HashMap::new(),
        }
    }

    fn load(&mut self, package: &String, name: &String) -> ClassFile {
        if let Some(class) = self.classes.get(name) {
            return class.clone();
        }

        info!("Loading class {name:?}");

        for path in &self.classpath {
            for dir_entry in path.read_dir().unwrap() {
                let dir_entry = dir_entry.unwrap();
                if dir_entry.file_name().into_string().unwrap() == format!("{name}.class") {
                    let class = ClassFile::new(&dir_entry.path());
                    self.classes
                        .insert(format!("{package}.{name}"), class.clone());
                    return class;
                }
            }
        }

        panic!("Unable to find class {package}.{name}");
    }
}

#[derive(Clone, Debug)]
struct ClassFile {
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
    fn new(p: &Path) -> ClassFile {
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
enum ConstantPoolInfo {
    Reserved,
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Class {
        name_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf {
        value: String,
    },
}

impl ConstantPoolInfo {
    fn new(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut tag = [0u8; 1];
        c.read_exact(&mut tag).unwrap();
        match tag[0] {
            1 => ConstantPoolInfo::utf8(c),
            7 => ConstantPoolInfo::class(c),
            8 => ConstantPoolInfo::string(c),
            9 => ConstantPoolInfo::field_ref(c),
            10 => ConstantPoolInfo::method_ref(c),
            12 => ConstantPoolInfo::name_and_type(c),
            t => panic!("invalid constant pool tag {t}"),
        }
    }

    fn class(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut name_index = [0u8; 2];
        c.read_exact(&mut name_index).unwrap();
        let name_index = u16::from_be_bytes(name_index);

        ConstantPoolInfo::Class { name_index }
    }

    fn method_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut class_index = [0u8; 2];
        c.read_exact(&mut class_index).unwrap();
        let class_index = u16::from_be_bytes(class_index);

        let mut name_and_type_index = [0u8; 2];
        c.read_exact(&mut name_and_type_index).unwrap();
        let name_and_type_index = u16::from_be_bytes(name_and_type_index);

        ConstantPoolInfo::MethodRef {
            class_index,
            name_and_type_index,
        }
    }

    fn field_ref(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut class_index = [0u8; 2];
        c.read_exact(&mut class_index).unwrap();
        let class_index = u16::from_be_bytes(class_index);

        let mut name_and_type_index = [0u8; 2];
        c.read_exact(&mut name_and_type_index).unwrap();
        let name_and_type_index = u16::from_be_bytes(name_and_type_index);

        ConstantPoolInfo::FieldRef {
            class_index,
            name_and_type_index,
        }
    }

    fn string(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut string_index = [0u8; 2];
        c.read_exact(&mut string_index).unwrap();
        let string_index = u16::from_be_bytes(string_index);

        ConstantPoolInfo::String { string_index }
    }

    fn name_and_type(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut name_index = [0u8; 2];
        c.read_exact(&mut name_index).unwrap();
        let name_index = u16::from_be_bytes(name_index);

        let mut descriptor_index = [0u8; 2];
        c.read_exact(&mut descriptor_index).unwrap();
        let descriptor_index = u16::from_be_bytes(descriptor_index);

        ConstantPoolInfo::NameAndType {
            name_index,
            descriptor_index,
        }
    }

    fn utf8(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut length = [0u8; 2];
        c.read_exact(&mut length).unwrap();
        let length = u16::from_be_bytes(length);

        let mut value = vec![0u8; length as usize];
        c.read_exact(&mut value).unwrap();
        let value = String::from_utf8(value).unwrap();

        ConstantPoolInfo::Utf { value }
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

#[derive(Clone, Debug)]
struct Method {
    access_flags: Vec<MethodFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl Method {
    fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Method {
        let mut access_flags = [0u8; 2];
        c.read_exact(&mut access_flags).unwrap();
        let access_flags = MethodFlag::flags(u16::from_be_bytes(access_flags));

        let mut name_index = [0u8; 2];
        c.read_exact(&mut name_index).unwrap();
        let name_index = u16::from_be_bytes(name_index);

        let mut descriptor_index = [0u8; 2];
        c.read_exact(&mut descriptor_index).unwrap();
        let descriptor_index = u16::from_be_bytes(descriptor_index);

        let mut attributes_count = [0u8; 2];
        c.read_exact(&mut attributes_count).unwrap();
        let attributes_count = u16::from_be_bytes(attributes_count);

        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(c, constant_pool));
        }

        Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }
}

#[derive(Clone, Debug)]
struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

#[derive(Clone, Debug)]
enum Attribute {
    Code {
        name_index: u16,
        length: u32,
        max_stacks: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table_length: u16,
        attributes: Vec<Attribute>,
    },
    LineNumberTable {
        name_index: u16,
        length: u32,
        table: Vec<LineNumberTableEntry>,
    },

    SourceFile {
        name_index: u16,
        length: u32,
        source_file_index: u16,
    },
}

impl Attribute {
    fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Attribute {
        let mut attribute_name_index = [0u8; 2];
        c.read_exact(&mut attribute_name_index).unwrap();
        let name_index = u16::from_be_bytes(attribute_name_index);

        let mut attribute_length = [0u8; 4];
        c.read_exact(&mut attribute_length).unwrap();
        let length = u32::from_be_bytes(attribute_length);

        let pool_info = constant_pool.get(name_index as usize).unwrap();

        if let ConstantPoolInfo::Utf { value } = pool_info {
            match value.as_str() {
                "Code" => Attribute::code(c, name_index, length, constant_pool),
                "LineNumberTable" => Attribute::line_number_table(c, name_index, length),
                "SourceFile" => Attribute::source_file(c, name_index, length),
                i => panic!("unknown attribute {i}"),
            }
        } else {
            panic!(
                "attribute_name_index must refer to Utf8 entry in constant pool, is {:?}",
                pool_info
            );
        }
    }

    fn source_file(c: &mut Cursor<&Vec<u8>>, name_index: u16, length: u32) -> Attribute {
        let mut source_file_index = [0u8; 2];
        c.read_exact(&mut source_file_index).unwrap();
        let source_file_index = u16::from_be_bytes(source_file_index);

        Attribute::SourceFile {
            name_index,
            length,
            source_file_index,
        }
    }

    fn line_number_table(c: &mut Cursor<&Vec<u8>>, name_index: u16, length: u32) -> Attribute {
        let mut table_length = [0u8; 2];
        c.read_exact(&mut table_length).unwrap();
        let table_length = u16::from_be_bytes(table_length);

        let mut table = Vec::new();
        for _ in 0..table_length {
            let mut start_pc = [0u8; 2];
            c.read_exact(&mut start_pc).unwrap();
            let start_pc = u16::from_be_bytes(start_pc);

            let mut line_number = [0u8; 2];
            c.read_exact(&mut line_number).unwrap();
            let line_number = u16::from_be_bytes(line_number);

            table.push(LineNumberTableEntry {
                start_pc,
                line_number,
            });
        }

        Attribute::LineNumberTable {
            name_index,
            length,
            table,
        }
    }

    fn code(
        c: &mut Cursor<&Vec<u8>>,
        name_index: u16,
        length: u32,
        constant_pool: &[ConstantPoolInfo],
    ) -> Attribute {
        let mut max_stacks = [0u8; 2];
        c.read_exact(&mut max_stacks).unwrap();
        let max_stacks = u16::from_be_bytes(max_stacks);

        let mut max_locals = [0u8; 2];
        c.read_exact(&mut max_locals).unwrap();
        let max_locals = u16::from_be_bytes(max_locals);

        let mut code_length = [0u8; 4];
        c.read_exact(&mut code_length).unwrap();
        let code_length = u32::from_be_bytes(code_length);

        assert!(code_length > 0);

        let mut code = vec![0u8; code_length as usize];
        c.read_exact(&mut code).unwrap();

        let mut exception_table_length = [0u8; 2];
        c.read_exact(&mut exception_table_length).unwrap();
        let exception_table_length = u16::from_be_bytes(exception_table_length);

        assert_eq!(exception_table_length, 0, "exceptions are not implemented");

        let mut attributes_count = [0u8; 2];
        c.read_exact(&mut attributes_count).unwrap();
        let attributes_count = u16::from_be_bytes(attributes_count);

        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(c, constant_pool));
        }

        Attribute::Code {
            name_index,
            length,
            max_stacks,
            max_locals,
            code,
            exception_table_length,
            attributes,
        }
    }
}

#[derive(Clone, Debug)]
enum MethodFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Native,
    Abstract,
}

impl MethodFlag {
    fn flags(val: u16) -> Vec<MethodFlag> {
        let mut flags = Vec::new();

        if (val & 0x0001) != 0 {
            flags.push(MethodFlag::Public);
        }

        if (val & 0x0002) != 0 {
            flags.push(MethodFlag::Private);
        }

        if (val & 0x0004) != 0 {
            flags.push(MethodFlag::Protected);
        }

        if (val & 0x0008) != 0 {
            flags.push(MethodFlag::Static);
        }

        if (val & 0x0010) != 0 {
            flags.push(MethodFlag::Final);
        }

        if (val & 0x0020) != 0 {
            flags.push(MethodFlag::Synchronized);
        }

        if (val & 0x0100) != 0 {
            flags.push(MethodFlag::Native);
        }

        if (val & 0x0400) != 0 {
            flags.push(MethodFlag::Abstract);
        }

        flags
    }
}
