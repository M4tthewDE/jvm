// https://www.cs.miami.edu/home/burt/reference/java/language_vm_specification.pdf
// https://blogs.oracle.com/javamagazine/post/how-the-jvm-locates-loads-and-runs-libraries
// https://www.mobilefish.com/services/java_decompiler/java_decompiler.php

use std::path::PathBuf;

use clap::Parser;
use loader::{class_path::ClassPath, ClassLoader};

mod loader;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    classpath: Option<Vec<PathBuf>>,
    #[arg(short, long)]
    main_class: String,
}

fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();

    let class_path = ClassPath::load(cli.classpath.unwrap_or_default());
    let mut class_loader = ClassLoader::new(class_path);
    class_loader.load(&"".to_string(), &cli.main_class);
}
