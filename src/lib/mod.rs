use std::{fmt::Display, path::PathBuf};

use executor::Executor;
use loader::{class_path::ClassPath, ClassLoader};

mod executor;
mod loader;
mod parser;

pub fn run(class_path: Vec<PathBuf>, main_class: ClassName) {
    let class_path = ClassPath::load(class_path);
    let mut class_loader = ClassLoader::new(class_path);
    class_loader.load_main(Package::new("".to_string()), main_class.clone());

    let mut executor = Executor::new(class_loader);
    executor.execute(Package::new("".to_string()), main_class);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ClassName {
    name: String,
}

impl ClassName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Package {
    name: String,
}

impl Package {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
