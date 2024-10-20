use std::path::PathBuf;

use jvm::{ClassIdentifier, ClassName, Package};

#[test]
fn test_main() {
    let package = Package::default();
    let name = ClassName::new("Main".to_string());
    jvm::run(
        vec![PathBuf::from("testdata/")],
        ClassIdentifier::new(package, name),
    );
}

#[test]
#[should_panic(expected = "No main method in class .MainNoMain")]
fn test_main_no_main() {
    let package = Package::default();
    let name = ClassName::new("MainNoMain".to_string());
    jvm::run(
        vec![PathBuf::from("testdata/")],
        ClassIdentifier::new(package, name),
    );
}
