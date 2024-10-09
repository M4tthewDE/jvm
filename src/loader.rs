use std::{collections::HashMap, path::PathBuf};

use tracing::info;

use crate::parser::class::ClassFile;

pub struct ClassLoader {
    classpath: Vec<PathBuf>,
    classes: HashMap<String, ClassFile>,
}

impl ClassLoader {
    pub fn new(cp: Vec<String>) -> ClassLoader {
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

    pub fn load(&mut self, package: &String, name: &String) -> ClassFile {
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
