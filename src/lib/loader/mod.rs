use std::collections::HashMap;

use class_path::ClassPath;
use tracing::info;

use crate::{parser::class::ClassFile, ClassName, Package};

pub mod class_path;

pub struct ClassLoader {
    class_path: ClassPath,
    classes: HashMap<(Package, ClassName), ClassFile>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> ClassLoader {
        ClassLoader {
            class_path,
            classes: HashMap::new(),
        }
    }

    pub fn load(&mut self, package: Package, name: ClassName) -> ClassFile {
        if let Some(c) = self.classes.get(&(package.clone(), name.clone())) {
            return c.clone();
        }

        info!("Loading class {name:?}");

        let data = self
            .class_path
            .find(&package, &name)
            .unwrap_or_else(|| panic!("unable to find class {package}.{name} in classpath"));
        let class = ClassFile::new(&data, package.clone(), name.clone());

        self.classes.insert((package, name), class.clone());
        class
    }

    pub fn load_main(&mut self, package: Package, name: ClassName) {
        if self.classes.contains_key(&(package.clone(), name.clone())) {
            return;
        }

        info!("Loading main class {name:?}");

        let data = self.class_path.find(&package, &name).unwrap();
        let class = ClassFile::new(&data, package.clone(), name.clone());
        if !class.has_main() {
            panic!("No main method in class {name}");
        }
        self.classes.insert((package, name), class.clone());
    }
}
