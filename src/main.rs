// https://www.cs.miami.edu/home/burt/reference/java/language_vm_specification.pdf
// https://blogs.oracle.com/javamagazine/post/how-the-jvm-locates-loads-and-runs-libraries
// https://www.mobilefish.com/services/java_decompiler/java_decompiler.php

use std::path::PathBuf;

use clap::Parser;

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
    jvm::run(cli.classpath.unwrap(), &cli.main_class)
}
