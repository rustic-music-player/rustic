use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    generate_header(&crate_dir);
    build_integration_tests();
}

fn generate_header(crate_dir: &str) {
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_version(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}

fn build_integration_tests() {
    let tool = cc::Build::new().get_compiler();
    let compiler = tool.path();
    build_integration_test(compiler, "sync_http_interop");
    build_integration_test(compiler, "sync_http_interop");
}

fn build_integration_test(compiler: &Path, test: &str) {
    let success = Command::new(compiler)
        .arg(&format!("tests/{}.c", test))
        .arg("-lrustic_ffi_client")
        .arg("-o")
        .arg(&format!("tests/{}.out", test))
        .arg("-B")
        .arg("../../target/debug")
        .status()
        .expect("failed to compile")
        .success();
    assert!(success);
}
