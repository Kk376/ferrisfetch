use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "A fast, lightweight Linux system information fetch tool",
        ))
        .stdout(predicate::str::contains("--modules"))
        .stdout(predicate::str::contains("--list-modules"));
}

#[test]
fn test_list_modules_flag() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.arg("--list-modules");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("os"))
        .stdout(predicate::str::contains("kernel"))
        .stdout(predicate::str::contains("cpu"))
        .stdout(predicate::str::contains("memory"))
        .stdout(predicate::str::contains("uptime"))
        .stdout(predicate::str::contains("shell"))
        .stdout(predicate::str::contains("disk"));
}

#[test]
fn test_no_color_flag() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.arg("--no-color");
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(!stdout.contains("\x1b["));
}

#[test]
fn test_custom_modules_filtering() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.args(["--no-color", "--no-logo", "-m", "os,kernel"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("OS:"));
    assert!(stdout.contains("Kernel:"));
    assert!(!stdout.contains("CPU:"));
    assert!(!stdout.contains("Memory:"));
}

#[test]
fn test_disable_module_flag() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.args([
        "--no-color",
        "--no-logo",
        "-m",
        "os,kernel,cpu",
        "-d",
        "cpu",
    ]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("OS:"));
    assert!(stdout.contains("Kernel:"));
    assert!(!stdout.contains("CPU:"));
}

#[test]
fn test_logo_override_flag() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.args(["--no-color", "--logo", "arch", "-m", "os"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("/\\"));
    assert!(stdout.contains("OS:"));
}

#[test]
fn test_no_logo_flag() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.args(["--no-color", "--no-logo", "-m", "os"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(
        stdout.trim(),
        format!("OS: {}", ferrisfetch::modules::os::detect_os().display_name)
    );
}

#[test]
fn test_disk_path_flag() {
    let mut cmd = Command::cargo_bin("ferrisfetch").unwrap();
    cmd.args(["--no-color", "--no-logo", "-m", "disk", "--disk-path", "/"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("Disk:"));
    assert!(stdout.contains('%'));
}
