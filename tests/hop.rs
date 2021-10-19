use assert_cmd::Command; // Add methods on commands

use std::fs;
use std::io;
use tempfile::{tempdir};

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hop")?;

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    cmd.arg("-V");
    cmd.assert()
        .success()
        .stdout(format!("Hop {}\n", VERSION));

    Ok(())
}

 #[test]
fn creates_hop_home() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let hop_home = hop_home_temp.as_path();

    //ensure hop home does not exist before running the exec
    fs::metadata(hop_home).expect_err(&format!("Found hop_home: {}", hop_home.to_string_lossy()));

    let mut cmd = Command::cargo_bin("hop")?;
    cmd
    .arg("-c")
    .arg(hop_home.as_os_str())
    .arg("-l")
    .assert()
    .success();

    fs::metadata(hop_home).expect(&format!("Could not find hop_home: {}", hop_home.to_string_lossy()));

    Ok(())
}

 #[test]
fn create_links() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let target_dir_temp = working_dir.path().join("somedir");
    let hop_home = hop_home_temp.as_path();
    let target_dir = target_dir_temp.as_path();

    fs::create_dir_all(target_dir)?;

    let mut cmd = Command::cargo_bin("hop")?;
    cmd
    .arg("-c")
    .arg(hop_home.as_os_str())
    .arg("-m")
    .arg("blee")
    .arg(target_dir.as_os_str())
    .assert()
    .success();

    fs::metadata(target_dir).expect(&format!("Could not find target dir: {}", target_dir.to_string_lossy()));

    let entries = fs::read_dir(hop_home)?.map(|res| res.map(|d| d.file_name())).collect::<Result<Vec<_>, io::Error>>()?;
    assert_eq!(entries, vec!["blee"]);

    Ok(())
}
