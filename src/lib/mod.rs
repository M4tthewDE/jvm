use std::{fmt::Display, path::PathBuf};

use executor::Executor;
use loader::{class_path::ClassPath, ClassLoader};

// TODO: restrict after loader is in executor
pub mod executor;
mod loader;
mod parser;

pub fn run(class_path: Vec<PathBuf>, main_class: ClassIdentifier) {
    let class_path = ClassPath::load(class_path);
    let mut class_loader = ClassLoader::new(class_path);
    class_loader.load_main(main_class.clone());

    let mut executor = Executor::new(class_loader);
    executor.execute(main_class);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ClassName {
    pub name: String,
}

impl ClassName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Package {
    pub name: String,
}

impl Package {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ClassIdentifier {
    pub package: Package,
    pub name: ClassName,
}

impl ClassIdentifier {
    pub fn new(package: Package, name: ClassName) -> Self {
        Self { package, name }
    }
}

impl Display for ClassIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.package, self.name)
    }
}
