use std::{env, fs::File, io::Read, path::PathBuf};

use zip::ZipArchive;

pub struct ClassPath {
    paths: Vec<PathBuf>,
}

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

    pub fn find(&self, package: &str, name: &str) -> Option<Vec<u8>> {
        let file_name = format!("{name}.class");

        for path in &self.paths {
            for dir_entry in path.read_dir().unwrap() {
                let path = dir_entry.unwrap().path();

                if let Some(extension) = path.extension() {
                    if extension == "jmod" {
                        if let Some(p) = Self::find_in_jmod(&path, package, name) {
                            return Some(p);
                        }
                    }
                }

                if *path.file_name().unwrap() == *file_name {
                    return std::fs::read(path).ok();
                }
            }
        }

        None
    }

    fn find_in_jmod(path: &PathBuf, package: &str, name: &str) -> Option<Vec<u8>> {
        let file = File::open(path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        let package = package.replace(".", "/");
        let file_path = format!("classes/{package}/{name}.class");

        let data = match archive.by_name(&file_path) {
            Ok(mut f) => {
                dbg!(f.name());
                let mut data = Vec::new();
                f.read_to_end(&mut data).unwrap();
                Some(data)
            }
            Err(_) => None,
        };

        data
    }
}
