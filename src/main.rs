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

#[derive(Clone)]
struct ClassFile {}

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
        for _ in 0..constant_pool_count {
            let cp_info = ConstantPoolInfo::new(&mut c);
            info!("Constant pool info: {cp_info:?}");
            constant_pool.push(cp_info);
        }

        todo!("class file loading");
    }
}

#[derive(Clone, Debug)]
enum ConstantPoolInfo {
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
}

impl ConstantPoolInfo {
    fn new(c: &mut Cursor<&Vec<u8>>) -> ConstantPoolInfo {
        let mut tag = [0u8; 1];
        c.read_exact(&mut tag).unwrap();
        match tag[0] {
            10 => ConstantPoolInfo::method_ref(c),
            t => panic!("invalid constant pool tag {t}"),
        }
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
}
