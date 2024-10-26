use anyhow::{bail, Result};
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

    pub fn load(&mut self, class_identifier: ClassIdentifier) -> Result<Class> {
        if let Some(c) = self.classes.get(&class_identifier) {
            return Ok(c.clone());
        }

        info!("Loading class {class_identifier}");

        let data = self.class_path.find(&class_identifier)?;
        let class_file = ClassFile::new(&data, class_identifier.clone())?;
        let class = Class::new(class_file)?;

        self.classes.insert(class_identifier, class.clone());
        Ok(class)
    }

    pub fn load_main(&mut self, class_identifier: ClassIdentifier) -> Result<()> {
        if self.classes.contains_key(&class_identifier) {
            return Ok(());
        }

        info!("Loading main class {class_identifier}");

        let data = self.class_path.find(&class_identifier)?;
        let class_file = ClassFile::new(&data, class_identifier.clone())?;
        let class = Class::new(class_file)?;
        if !class.has_main() {
            bail!("No main method in class {class_identifier}");
        }
        self.classes.insert(class_identifier, class.clone());
        Ok(())
    }
}
