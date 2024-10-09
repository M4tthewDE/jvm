use std::{collections::HashMap, path::PathBuf};

use tracing::info;

use crate::parser::class::ClassFile;

pub struct ClassLoader {
    classpath: Vec<PathBuf>,
    classes: HashMap<String, ClassFile>,
}

impl ClassLoader {
    pub fn new(classpath: Vec<PathBuf>) -> ClassLoader {
        ClassLoader::validate_classpath(&classpath);

        ClassLoader {
            classpath,
            classes: HashMap::new(),
        }
    }

    fn validate_classpath(classpath: &[PathBuf]) {
        for p in classpath {
            if !p.exists() {
                panic!("invalid classpath: {p:?}");
            }
        }
    }

    fn key(package: &str, name: &str) -> String {
        format!("{package}.{name}")
    }

    pub fn load(&mut self, package: &str, name: &str) {
        if self.classes.contains_key(&ClassLoader::key(package, name)) {
            return;
        }

        self.load_class(package, name);
    }

    fn load_class(&mut self, package: &str, name: &str) {
        info!("Loading class {name:?}");
        let class = ClassFile::new(&self.find_path(name).unwrap());
        self.classes
            .insert(format!("{package}.{name}"), class.clone());
    }

    fn find_path(&self, name: &str) -> Option<PathBuf> {
        let file_name = format!("{name}.class");

        for path in &self.classpath {
            for dir_entry in path.read_dir().unwrap() {
                let dir_entry = dir_entry.unwrap();
                if dir_entry.file_name().into_string().unwrap() == file_name {
                    return Some(dir_entry.path());
                }
            }
        }

        None
    }
}
