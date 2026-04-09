use assert_cmd::Command;

#[allow(deprecated)]
fn bin() -> Command {
    Command::cargo_bin("ftime").unwrap()
}

#[test]
fn help_describes_default_mode_and_ignore_sources() {
    let output = bin().arg("--help").output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Default output is the human bucket view."));
    assert!(stdout.contains(
        "Disable ignore rules (built-in, FTIME_IGNORE, ~/.ftimeignore, and local .ftimeignore)"
    ));
}
