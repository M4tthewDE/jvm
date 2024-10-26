// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf
// https://github.com/openjdk/jdk17
// https://blogs.oracle.com/javamagazine/post/how-the-jvm-locates-loads-and-runs-libraries

use std::path::PathBuf;

use clap::Parser;
use jvm::{ClassIdentifier, ClassName, Package};
use tracing::{error, info};
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    classpath: Vec<PathBuf>,
    #[arg(short, long)]
    main_class: String,
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();
    let package = Package::default();
    let name = ClassName::new(cli.main_class);

    match jvm::run(cli.classpath, ClassIdentifier::new(package, name)) {
        Ok(_) => info!("Done!"),
        Err(err) => error!("{err:?}"),
    }
}
