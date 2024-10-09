use std::path::PathBuf;

pub struct ClassPath {
    paths: Vec<PathBuf>,
}

impl ClassPath {
    pub fn load(paths: Vec<PathBuf>) -> ClassPath {
        for p in &paths {
            if !p.exists() {
                panic!("invalid classpath: {p:?}");
            }
        }

        ClassPath { paths }
    }

    pub fn find(&self, name: &str) -> Option<PathBuf> {
        let file_name = format!("{name}.class");

        for path in &self.paths {
            for dir_entry in path.read_dir().unwrap() {
                let dir_entry = dir_entry.unwrap();
                if dir_entry.file_name().into_string().unwrap() == file_name {
                    return Some(dir_entry.path());
                }
            }
        }

        None
    }
}
