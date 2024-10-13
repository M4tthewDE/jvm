use std::path::PathBuf;

use executor::Executor;
use loader::{class_path::ClassPath, ClassLoader};

mod executor;
mod loader;
mod parser;

pub fn run(class_path: Vec<PathBuf>, main_class: &str) {
    let class_path = ClassPath::load(class_path);
    let mut class_loader = ClassLoader::new(class_path);
    class_loader.load_main("", main_class);

    let mut executor = Executor::new(class_loader);

    executor.execute("", main_class);
}
