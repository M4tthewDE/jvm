use std::path::PathBuf;

#[test]
#[should_panic(expected = "not yet implemented: implement invoke_static for native methods")]
fn test_main() {
    jvm::run(vec![PathBuf::from("testdata/")], "Main");
}

#[test]
#[should_panic(expected = "No main method in class MainNoMain")]
fn test_main_no_main() {
    jvm::run(vec![PathBuf::from("testdata/")], "MainNoMain");
}
