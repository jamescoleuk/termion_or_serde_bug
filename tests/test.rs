use std::process::{Command, Output};

#[test]
fn run_no_param() {
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}
