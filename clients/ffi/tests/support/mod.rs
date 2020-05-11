use std::process::Command;

pub fn run_test(name: &str) {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = format!("{}/tests/{}.out", crate_dir, name);
    let status_code = Command::new(&path)
        .status()
        .expect("failed to run")
        .code()
        .expect("command got interrupted");

    assert_eq!(status_code, 0, "command returned invalid status code");
}
