use std::{env, fs::File, io::Read, path::PathBuf};

use anyhow::{bail, Context, Result};
use zip::ZipArchive;

use crate::ClassIdentifier;

pub struct ClassPath {
    paths: Vec<PathBuf>,
}

const JMOD_FILES: [&str; 1] = ["java.base.jmod"];

impl ClassPath {
    pub fn load(mut paths: Vec<PathBuf>) -> Result<ClassPath> {
        for p in &paths {
            if !p.exists() {
                bail!("invalid classpath: {p:?}");
            }
        }

        let java_home = env::var("JAVA_HOME")?;
        let jmods = PathBuf::from(java_home).join("jmods");
        if !jmods.exists() {
            bail!("jmods/ not found in $JAVA_HOME");
        }
        paths.push(jmods);

        Ok(ClassPath { paths })
    }

    pub fn find(&self, identifier: &ClassIdentifier) -> Result<Vec<u8>> {
        let file_name = format!("{}.class", identifier.name);

        for path in &self.paths {
            for dir_entry in path.read_dir()? {
                let path = dir_entry?.path();

                if JMOD_FILES.contains(
                    &path
                        .file_name()
                        .context(format!("invalid file_name at {path:?}"))?
                        .to_str()
                        .context(format!("invalid file_name at {path:?}"))?,
                ) {
                    if let Some(p) = Self::find_in_jmod(&path, identifier)? {
                        return Ok(p);
                    }
                }

                if *path
                    .file_name()
                    .context(format!("invalid file_name at {path:?}"))?
                    == *file_name
                {
                    return Ok(std::fs::read(path)?);
                }
            }
        }

        bail!("class {identifier} not found");
    }

    fn find_in_jmod(path: &PathBuf, identifier: &ClassIdentifier) -> Result<Option<Vec<u8>>> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;

        let package = identifier.package.name.replace(".", "/");
        let file_path = format!("classes/{package}/{}.class", identifier.name);

        let mut data = Vec::new();
        archive.by_name(&file_path)?.read_to_end(&mut data)?;
        Ok(Some(data))
    }
}
