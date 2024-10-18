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

    pub fn load(&mut self, package: &str, name: &str) -> ClassFile {
        if let Some(c) = self.classes.get(&key(package, name)) {
            return c.clone();
        }

        info!("Loading class {name:?}");

        let data = self
            .class_path
            .find(package, name)
            .unwrap_or_else(|| panic!("unable to find class {package}.{name} in classpath"));
        let class = ClassFile::new(&data, package.to_string(), name.to_string());

        self.classes.insert(key(package, name), class.clone());
        class
    }

    pub fn load_main(&mut self, package: &str, name: &str) {
        if self.classes.contains_key(&key(package, name)) {
            return;
        }

        info!("Loading main class {name:?}");

        let data = self.class_path.find("", name).unwrap();
        let class = ClassFile::new(&data, package.to_string(), name.to_string());
        if !class.has_main() {
            panic!("No main method in class {name}");
        }
        self.classes.insert(key(package, name), class.clone());
    }
}

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}
