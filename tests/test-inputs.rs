use std::process::{Command, Stdio};

#[test]
fn test_input() -> anyhow::Result<()> {
    let child = Command::new(env!("CARGO_BIN_EXE_moonwave"))
        .args(&["extract", "test-input/init.lua"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("NO_COLOR", "1")
        .spawn()?;

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    insta::assert_snapshot!("stdout", stdout);
    insta::assert_snapshot!("stderr", stderr);

    Ok(())
}
