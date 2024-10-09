use std::{collections::HashMap, path::PathBuf};

use class_path::ClassPath;
use tracing::info;

use crate::parser::class::ClassFile;

pub mod class_path;

pub struct ClassLoader {
    class_path: ClassPath,
    classes: HashMap<String, ClassFile>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> ClassLoader {
        ClassLoader {
            class_path,
            classes: HashMap::new(),
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

        let path = self.class_path.find(name).unwrap();
        let class = ClassFile::new(&path);
        self.classes
            .insert(format!("{package}.{name}"), class.clone());
    }
}
