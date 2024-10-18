use std::{env, fs::File, io::Read, path::PathBuf};

use zip::ZipArchive;

use crate::{ClassName, Package};

pub struct ClassPath {
    paths: Vec<PathBuf>,
}

const JMOD_FILES: [&str; 1] = ["java.base.jmod"];

impl ClassPath {
    pub fn load(mut paths: Vec<PathBuf>) -> ClassPath {
        for p in &paths {
            if !p.exists() {
                panic!("invalid classpath: {p:?}");
            }
        }

        let java_home = env::var("JAVA_HOME").expect("JAVA_HOME is not set");
        let jmods = PathBuf::from(java_home).join("jmods");
        assert!(jmods.exists(), "jmods/ not found in $JAVA_HOME");
        paths.push(jmods);

        ClassPath { paths }
    }

    pub fn find(&self, package: &Package, name: &ClassName) -> Option<Vec<u8>> {
        let file_name = format!("{name}.class");

        for path in &self.paths {
            for dir_entry in path.read_dir().unwrap() {
                let path = dir_entry.unwrap().path();

                if JMOD_FILES.contains(&path.file_name().unwrap().to_str().unwrap()) {
                    if let Some(p) = Self::find_in_jmod(&path, package, name) {
                        return Some(p);
                    }
                }

                if *path.file_name().unwrap() == *file_name {
                    return std::fs::read(path).ok();
                }
            }
        }

        None
    }

    fn find_in_jmod(path: &PathBuf, package: &Package, name: &ClassName) -> Option<Vec<u8>> {
        let file = File::open(path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        let package = package.name.replace(".", "/");
        let file_path = format!("classes/{package}/{name}.class");

        let data = match archive.by_name(&file_path) {
            Ok(mut f) => {
                let mut data = Vec::new();
                f.read_to_end(&mut data).unwrap();
                Some(data)
            }
            Err(_) => None,
        };

        data
    }
}
