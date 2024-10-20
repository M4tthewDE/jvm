use std::collections::HashMap;

use class_path::ClassPath;
use tracing::info;

use crate::{executor::class::Class, parser::class::ClassFile, ClassIdentifier};

pub mod class_path;

pub struct ClassLoader {
    class_path: ClassPath,
    classes: HashMap<ClassIdentifier, Class>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> ClassLoader {
        ClassLoader {
            class_path,
            classes: HashMap::new(),
        }
    }

    pub fn load(&mut self, class_identifier: ClassIdentifier) -> Class {
        if let Some(c) = self.classes.get(&class_identifier) {
            return c.clone();
        }

        info!("Loading class {class_identifier}");

        let data = self
            .class_path
            .find(&class_identifier)
            .unwrap_or_else(|| panic!("unable to find class {class_identifier} in classpath"));
        let class_file = ClassFile::new(&data, class_identifier.clone());
        let class = Class::new(class_file);

        self.classes.insert(class_identifier, class.clone());
        class
    }

    pub fn load_main(&mut self, class_identifier: ClassIdentifier) {
        if self.classes.contains_key(&class_identifier) {
            return;
        }

        info!("Loading main class {class_identifier}");

        let data = self.class_path.find(&class_identifier).unwrap();
        let class_file = ClassFile::new(&data, class_identifier.clone());
        let class = Class::new(class_file);
        if !class.has_main() {
            panic!("No main method in class {class_identifier}");
        }
        self.classes.insert(class_identifier, class.clone());
    }
}
