// https://www.cs.miami.edu/home/burt/reference/java/language_vm_specification.pdf
// https://blogs.oracle.com/javamagazine/post/how-the-jvm-locates-loads-and-runs-libraries

use std::{
    collections::HashMap,
    io::Cursor,
    path::{Path, PathBuf},
};

use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    classpath: Option<Vec<String>>,
    #[arg(short, long)]
    main_class: String,
}

fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();

    let mut class_loader = ClassLoader::new(cli.classpath.unwrap_or_default());
    class_loader.load(&"".to_string(), &cli.main_class);
}

struct ClassLoader {
    classpath: Vec<PathBuf>,
    classes: HashMap<String, ClassFile>,
}

impl ClassLoader {
    fn new(cp: Vec<String>) -> ClassLoader {
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

    fn load(&mut self, package: &String, name: &String) -> ClassFile {
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

#[derive(Clone)]
struct ClassFile {}

impl ClassFile {
    fn new(p: &Path) -> ClassFile {
        let bytes = std::fs::read(p).unwrap();
        let _c = Cursor::new(&bytes);
        todo!();
    }
}
