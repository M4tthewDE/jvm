// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf
// https://blogs.oracle.com/javamagazine/post/how-the-jvm-locates-loads-and-runs-libraries

use std::path::PathBuf;

use clap::Parser;
use jvm::ClassName;

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
    jvm::run(cli.classpath.unwrap(), ClassName::new(cli.main_class))
}
