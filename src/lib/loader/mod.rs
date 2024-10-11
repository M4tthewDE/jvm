use std::collections::HashMap;

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

    pub fn load(&mut self, package: &str, name: &str) {
        if self.classes.contains_key(&key(package, name)) {
            return;
        }

        info!("Loading class {name:?}");

        let path = self.class_path.find(name).unwrap();
        let class = ClassFile::new(&path);
        self.classes.insert(key(package, name), class.clone());
    }

    pub fn load_main(&mut self, package: &str, name: &str) {
        if self.classes.contains_key(&key(package, name)) {
            return;
        }

        info!("Loading main class {name:?}");

        let path = self.class_path.find(name).unwrap();
        let class = ClassFile::new(&path);
        verify_main_class(&class, name);
        self.classes.insert(key(package, name), class.clone());
    }
}

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}

fn verify_main_class(class: &ClassFile, name: &str) {
    for method in &class.methods {
        if method.is_main(&class.constant_pool) {
            return;
        }
    }

    panic!("No main method in class {name}");
}
