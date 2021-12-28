use std::process::{Command, Output};

#[test]
fn run_no_param() {
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    assert!(output.status.success());
}
