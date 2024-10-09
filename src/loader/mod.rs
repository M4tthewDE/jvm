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

        self.load_class(package, name);
    }

    fn load_class(&mut self, package: &str, name: &str) {
        info!("Loading class {name:?}");

        let path = self.class_path.find(name).unwrap();
        let class = ClassFile::new(&path);
        self.classes.insert(key(package, name), class.clone());
    }
}

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}
