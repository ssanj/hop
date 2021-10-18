use assert_cmd::Command; // Add methods on commands

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hop")?;

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    println!("VERSION:{}", VERSION);

    cmd.arg("-V");
    cmd.assert()
        .success()
        .stdout(format!("Hop {}\n", VERSION));

    Ok(())
}
